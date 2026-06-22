use fuser::{
    Errno, FileAttr, FileHandle, FileType, Filesystem, Generation, INodeNo, LockOwner, OpenFlags,
    ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use std::{
    ffi::OsStr,
    sync::LazyLock,
    time::{Duration, SystemTime},
};

const TTL: Duration = Duration::from_secs(1);

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

pub struct LoiFs;

impl Filesystem for LoiFs {
    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        let data = file_data(ino.0);

        match ino.0 {
            1 => reply.attr(&TTL, &attr(ino, FileType::Directory, 0)),
            3 | 4 | 5 => reply.attr(&TTL, &attr(ino, FileType::RegularFile, data.len() as u64)),
            _ => reply.error(Errno::from_i32(libc::ENOENT)),
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

        type DirEntry<'a> = (INodeNo, u64, FileType, &'a str);

        let entries: [DirEntry; 5] = [
            (INodeNo(1), 1, FileType::Directory, "."),
            (INodeNo(1), 2, FileType::Directory, ".."),
            (INodeNo(3), 3, FileType::RegularFile, "hello.rs"),
            (INodeNo(4), 4, FileType::RegularFile, "loi.rs"),
            (INodeNo(5), 5, FileType::RegularFile, "tran.rs"),
        ];

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
        _lock_owner: Option<LockOwner>,
        reply: ReplyData,
    ) {
        let data: &[u8] = match ino.0 {
            3 => r#"pub fn main() { println!("Hello"); }"#.as_bytes(),
            4 => r#"// Loi file content"#.as_bytes(),
            5 => r#"// Tran file content"#.as_bytes(),
            _ => {
                reply.error(Errno::from_i32(libc::ENOENT));
                return;
            }
        };

        // 2. Safe bounds checking
        if offset >= data.len() as u64 {
            reply.data(&[]); // Return EOF
        } else {
            let start = offset as usize;
            // Ensure we don't go past the end of our specific data slice
            let end = std::cmp::min(start + size as usize, data.len());

            reply.data(&data[start..end]);
        }
    }
    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        if parent.0 == 1 {
            match name.to_str() {
                Some("hello.rs") => {
                    reply.entry(
                        &TTL,
                        &attr(INodeNo(3), FileType::RegularFile, 30),
                        Generation(1),
                    );
                }
                Some("loi.rs") => {
                    reply.entry(
                        &TTL,
                        &attr(INodeNo(4), FileType::RegularFile, 20),
                        Generation(1),
                    );
                }
                Some("tran.rs") => {
                    reply.entry(
                        &TTL,
                        &attr(INodeNo(5), FileType::RegularFile, 20),
                        Generation(1),
                    );
                }
                _ => reply.error(Errno::from_i32(libc::ENOENT)),
            }
        } else {
            reply.error(Errno::from_i32(libc::ENOENT));
        }
    }
}
