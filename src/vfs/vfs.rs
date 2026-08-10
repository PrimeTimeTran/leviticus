use fuser::{
	BsdFileFlags, Errno, FileAttr, FileHandle, FileType, Filesystem, FopenFlags, Generation, INodeNo,
	LockOwner, OpenFlags, RenameFlags, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEntry,
	Request, WriteFlags,
};

use std::{
	collections::HashMap,
	ffi::OsStr,
	path::PathBuf,
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc, LazyLock, Mutex,
	},
	time::{Duration, SystemTime},
};

const TTL: Duration = Duration::from_secs(1);

static BASE_TIME: LazyLock<SystemTime> = LazyLock::new(|| SystemTime::now());

fn attr(ino: INodeNo, kind: FileType, size: u64) -> FileAttr {
	let perm = match kind {
		FileType::Directory => 0o755,
		FileType::RegularFile => 0o644,
		_ => 0o644,
	};

	FileAttr {
		ino,
		size,
		blocks: (size + 511) / 512,
		atime: *BASE_TIME,
		mtime: *BASE_TIME,
		ctime: *BASE_TIME,
		crtime: *BASE_TIME,
		kind,
		perm,
		nlink: 1,
		uid: 501,
		gid: 20,
		rdev: 0,
		flags: 0,
		blksize: 512,
	}
}

fn file_data(ino: u64) -> &'static [u8] {
	match ino {
		3 => b"pub fn main() { println!(\"Hello\"); }",
		4 => b"// Loi file content",
		5 => b"// Tran file content",
		_ => b"",
	}
}

struct FileNode {
	ino: u64,
	name: &'static str,
	data: &'static [u8],
}

static FILES: LazyLock<Vec<FileNode>> = LazyLock::new(|| {
	vec![
		FileNode {
			ino: 3,
			name: "hello.rs",
			data: b"pub fn main() { println!(\"Hello\"); }",
		},
		FileNode {
			ino: 4,
			name: "loi.rs",
			data: b"// Loi file content",
		},
		FileNode {
			ino: 5,
			name: "tran.rs",
			data: b"// Tran file content",
		},
	]
});

fn file_by_ino(ino: u64) -> Option<&'static FileNode> {
	FILES.iter().find(|f| f.ino == ino)
}

#[derive(Debug)]
pub struct LoiFs {
	pub cwd: String,
	pub next_ino: AtomicU64,
	pub files: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
	pub filenames: Arc<Mutex<HashMap<String, u64>>>,
}
impl Default for LoiFs {
	fn default() -> Self {
		let mut files_map = HashMap::new();
		let mut names_map = HashMap::new();
		let mut max_ino = 1;

		for file in FILES.iter() {
			files_map.insert(file.ino, file.data.to_vec());
			names_map.insert(file.name.to_string(), file.ino);

			if file.ino >= max_ino {
				max_ino = file.ino + 1;
			}
		}

		Self {
			cwd: "/Users/future/KB/project/app/loi/data".to_string(),
			next_ino: AtomicU64::new(max_ino),
			files: Arc::new(Mutex::new(files_map)),
			filenames: Arc::new(Mutex::new(names_map)),
		}
	}
}
impl Filesystem for LoiFs {
	fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
		let files = self.files.lock().unwrap();
		if ino.0 == 1 {
			reply.attr(&TTL, &attr(ino, FileType::Directory, 0));
		} else if let Some(data) = files.get(&ino.0) {
			reply.attr(&TTL, &attr(ino, FileType::RegularFile, data.len() as u64));
		} else {
			reply.error(Errno::from_i32(libc::ENOENT));
		}
	}
	fn readdir(
		&self,
		_req: &Request,
		ino: INodeNo,
		_fh: FileHandle,
		offset: u64,
		mut reply: ReplyDirectory,
	) {
		if ino.0 != 1 {
			reply.error(Errno::from_i32(libc::ENOENT));
			return;
		}

		let mut entries = vec![
			(INodeNo(1), 1, FileType::Directory, "."),
			(INodeNo(1), 2, FileType::Directory, ".."),
		];

		let filenames = self.filenames.lock().unwrap();
		for (i, (name, &inode)) in filenames.iter().enumerate() {
			entries.push((INodeNo(inode), (i + 3) as u64, FileType::RegularFile, name));
		}

		for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
			reply.add(entry.0, (i + 1) as u64, entry.2, entry.3);
		}
		reply.ok();
	}
	fn read(
		&self,
		_req: &Request,
		ino: INodeNo,
		_fh: FileHandle,
		offset: u64,
		size: u32,
		_flags: OpenFlags,
		_lock: Option<LockOwner>,
		reply: ReplyData,
	) {
		let files = self.files.lock().unwrap();
		if let Some(data) = files.get(&ino.0) {
			let start = offset as usize;
			if start < data.len() {
				let end = (start + size as usize).min(data.len());
				reply.data(&data[start..end]);
			} else {
				reply.data(&[]);
			}
		} else {
			reply.error(Errno::from_i32(libc::ENOENT))
		}
	}
	// Add this to your impl
	fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
		let name_str = name.to_string_lossy().to_string();

		let filenames = self.filenames.lock().unwrap();
		if let Some(&ino) = filenames.get(&name_str) {
			let attr = attr(INodeNo(ino), FileType::RegularFile, 0);
			reply.entry(&TTL, &attr, Generation(0));
		} else {
			reply.error(Errno::from_i32(libc::ENOENT));
		}
	}

	fn write(
		&self,
		_req: &Request,
		ino: INodeNo,
		_fh: FileHandle,
		offset: u64,
		data: &[u8],
		_write_flags: WriteFlags,
		_flags: OpenFlags,
		_lock_owner: Option<LockOwner>,
		reply: fuser::ReplyWrite,
	) {
		let new_data = data.to_vec();
		let name = {
			let mut files = self.files.lock().unwrap();
			let filenames = self.filenames.lock().unwrap();

			if let Some(file_content) = files.get_mut(&ino.0) {
				let offset = offset as usize;
				let data_len = data.len();

				if offset + data_len > file_content.len() {
					file_content.resize(offset + data_len, 0);
				}
				file_content[offset..offset + data_len].copy_from_slice(data);
				filenames
					.iter()
					.find(|(_, i)| **i == ino.0)
					.map(|(n, _)| n.clone())
			} else {
				None
			}
		};
		let content_to_save = {
			let files = self.files.lock().unwrap();
			files.get(&ino.0).cloned()
		};
		if let (Some(n), Some(data)) = (name, content_to_save) {
			let base = PathBuf::from(&self.cwd);
			let path = base.join(&n);
			let _ = std::fs::write(path, data);
		}
		reply.written(data.len() as u32);
	}
	fn setattr(
		&self,
		_req: &Request,
		ino: INodeNo,
		_mode: Option<u32>,
		_uid: Option<u32>,
		_gid: Option<u32>,
		_size: Option<u64>,
		_atime: Option<fuser::TimeOrNow>,
		_mtime: Option<fuser::TimeOrNow>,
		_ctime: Option<SystemTime>,
		_fh: Option<FileHandle>,
		_crtime: Option<SystemTime>,
		_chgtime: Option<SystemTime>,
		_bkuptime: Option<SystemTime>,
		_flags: Option<BsdFileFlags>,
		reply: ReplyAttr,
	) {
		reply.attr(&TTL, &attr(ino, FileType::RegularFile, 0));
	}
	fn create(
		&self,
		_req: &Request,
		_parent: INodeNo,
		name: &OsStr,
		_mode: u32,
		_umask: u32,
		flags: i32,
		reply: ReplyCreate,
	) {
		let ino = self.next_ino.fetch_add(1, Ordering::SeqCst);
		let name_str = name.to_string_lossy().into_owned();

		self.filenames.lock().unwrap().insert(name_str, ino);
		self.files.lock().unwrap().insert(ino, Vec::new());
		let attr = attr(INodeNo(ino), FileType::RegularFile, 0);
		reply.created(
			&TTL,
			&attr,
			Generation(1),
			FileHandle(ino),
			fuser::FopenFlags::from_bits_truncate(flags as u32),
		);
	}
	fn unlink(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: fuser::ReplyEmpty) {
		let name_str = name.to_string_lossy().to_string();
		let mut filenames = self.filenames.lock().unwrap();
		let mut files = self.files.lock().unwrap();

		if let Some(ino) = filenames.remove(&name_str) {
			files.remove(&ino);
			let base = PathBuf::from(&self.cwd);
			let new_path = base.join(&name_str);
			let _ = std::fs::remove_file(new_path);

			reply.ok();
		} else {
			reply.error(Errno::from_i32(libc::ENOENT));
		}
	}
	fn rename(
		&self,
		_req: &Request,
		_parent: INodeNo,
		name: &OsStr,
		_newparent: INodeNo,
		newname: &OsStr,
		_flags: RenameFlags,
		reply: fuser::ReplyEmpty,
	) {
		let old_name = name.to_string_lossy().to_string();
		let new_name = newname.to_string_lossy().to_string();

		let mut filenames = self.filenames.lock().unwrap();

		// 1. Update in-memory map
		if let Some(ino) = filenames.remove(&old_name) {
			filenames.insert(new_name.clone(), ino);
			let base = PathBuf::from(&self.cwd);
			let old_path = base.join(&old_name);
			let new_path = base.join(&new_name);
			let _ = std::fs::rename(old_path, new_path);

			reply.ok();
		} else {
			reply.error(Errno::from_i32(libc::ENOENT));
		}
	}
}
impl LoiFs {
	pub fn load_from_disk(path: &str) -> Self {
		let mut files_map = HashMap::new();
		let mut names_map = HashMap::new();
		let mut ino_counter = 10;
		if let Ok(entries) = std::fs::read_dir(path) {
			for entry in entries.flatten() {
				let name = entry.file_name().to_string_lossy().to_string();
				let content = std::fs::read(entry.path()).unwrap_or_default();

				let ino = ino_counter;
				files_map.insert(ino, content);
				names_map.insert(name, ino);
				ino_counter += 1;
			}
		}

		Self {
			cwd: "/Users/future/KB/project/app/loi/data/{}".to_string(),
			next_ino: AtomicU64::new(ino_counter),
			files: Arc::new(Mutex::new(files_map)),
			filenames: Arc::new(Mutex::new(names_map)),
		}
	}
}
