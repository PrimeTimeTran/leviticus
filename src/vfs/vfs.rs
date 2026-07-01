// use fuser::ll::flags::fopen_flags::FopenFlags;
use fuser::{
    BsdFileFlags, Errno, FileAttr, FileHandle, FileType, Filesystem, FopenFlags, Generation,
    INodeNo, LockOwner, OpenFlags, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEntry,
    Request, WriteFlags,
};

use std::{
    collections::HashMap,
    ffi::OsStr,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime},
};

const TTL: Duration = Duration::from_secs(1);

// Save file, create file

static BASE_TIME: LazyLock<SystemTime> = LazyLock::new(|| SystemTime::now());

fn attr(ino: INodeNo, kind: FileType, size: u64) -> FileAttr {
    FileAttr {
        ino,
        size,
        blocks: (size + 511) / 512,
        atime: *BASE_TIME,
        mtime: *BASE_TIME,
        ctime: *BASE_TIME,
        crtime: *BASE_TIME,
        kind,
        perm: 0o755,
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

// Read
fn file_by_ino(ino: u64) -> Option<&'static FileNode> {
    FILES.iter().find(|f| f.ino == ino)
}

#[derive(Debug)]
pub struct LoiFs {
    pub next_ino: AtomicU64,
    pub files: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
    pub filenames: Arc<Mutex<HashMap<String, u64>>>,
}
impl Default for LoiFs {
    fn default() -> Self {
        let mut files_map = HashMap::new();
        let mut names_map = HashMap::new();
        let mut max_ino = 1; // Start counter above the fixed ones

        // Iterate over your static FILES and insert them into the maps
        for file in FILES.iter() {
            files_map.insert(file.ino, file.data.to_vec());
            names_map.insert(file.name.to_string(), file.ino);

            // Keep track of the highest Inode so we don't overwrite static ones
            if file.ino >= max_ino {
                max_ino = file.ino + 1;
            }
        }

        Self {
            next_ino: AtomicU64::new(max_ino),
            files: Arc::new(Mutex::new(files_map)),
            filenames: Arc::new(Mutex::new(names_map)),
        }
    }
}
// impl Filesystem for LoiFs {

// }

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

        let mut entries: Vec<(INodeNo, u64, FileType, &str)> = Vec::new();

        // static entries
        entries.push((INodeNo(1), 1, FileType::Directory, "."));
        entries.push((INodeNo(1), 2, FileType::Directory, ".."));

        // dynamic entries
        for (i, file) in FILES.iter().enumerate() {
            entries.push((
                INodeNo(file.ino),
                (i as u64) + 3,
                FileType::RegularFile,
                file.name,
            ));
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
            // Slice the data based on offset and size
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

    // fn write(
    //     &self,
    //     _req: &Request,
    //     ino: INodeNo,
    //     _fh: FileHandle,
    //     offset: u64,
    //     data: &[u8],
    //     _write_flags: WriteFlags,
    //     _flags: OpenFlags,
    //     _lock_owner: Option<LockOwner>,
    //     reply: fuser::ReplyWrite,
    // ) {
    //     // 1. You receive the bytes in 'data'
    //     // 2. You would typically store these in a thread-safe structure (like a Mutex<HashMap<INodeNo, Vec<u8>>>)
    //     println!(
    //         "Received write for inode {}: {} bytes at offset {}",
    //         ino.0,
    //         data.len(),
    //         offset
    //     );

    //     // 3. Acknowledge the write
    //     reply.written(data.len() as u32);
    // }
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
        let mut files = self.files.lock().unwrap();
        if let Some(file_content) = files.get_mut(&ino.0) {
            let offset = offset as usize;
            let data_len = data.len();
            if offset + data_len > file_content.len() {
                file_content.resize(offset + data_len, 0);
            }

            file_content[offset..offset + data_len].copy_from_slice(data);

            println!("Successfully wrote {} bytes to inode {}", data_len, ino.0);
            reply.written(data_len as u32);
        } else {
            reply.error(Errno::from_i32(libc::ENOENT));
        }
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
        // When you run `touch filename`, this method is called.
        // You must update your stored metadata for this inode.
        reply.attr(&TTL, &attr(ino, FileType::RegularFile, 0)); // Return the new attributes
    }

    // fn create(
    //     &self,
    //     _req: &Request,
    //     _parent: INodeNo,
    //     name: &OsStr,
    //     _mode: u32,
    //     _umask: u32,
    //     flags: i32, // The kernel passes the actual flags here
    //     reply: ReplyCreate,
    // ) {
    //     // In a real app, you would add a new entry to your internal HashMap here.
    //     // For now, let's just log that the kernel requested it:
    //     println!("Kernel requested creation of: {:?}", name);

    //     let open_flags = fuser::FopenFlags::from_bits_truncate(flags as u32);
    //     let attr = attr(INodeNo(6), FileType::RegularFile, 0);
    //     reply.created(&TTL, &attr, Generation(1), FileHandle(1), open_flags);
    // }
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

        // 1. Store the new file in your maps
        self.filenames.lock().unwrap().insert(name_str, ino);
        self.files.lock().unwrap().insert(ino, Vec::new());

        // 2. Reply with the new unique Inode
        let attr = attr(INodeNo(ino), FileType::RegularFile, 0);
        reply.created(
            &TTL,
            &attr,
            Generation(1),
            FileHandle(ino),
            fuser::FopenFlags::from_bits_truncate(flags as u32),
        );
    }
}
