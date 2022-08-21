use clap::{Arg, Command};
use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};
use libc::ENOENT;
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
//use log::{debug, error, log_enabled, info, Level};

const TTL: Duration = Duration::from_secs(1); // 1 second

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};

const EMPTY_DIR_ATTR: FileAttr = FileAttr {
    ino: 3,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};



const HELLO_TXT_CONTENT: &str = "Hello World!\n";

const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};

const HELLO2_TXT_CONTENT: &str = "Hello World2!\n";

const HELLO2_TXT_ATTR: FileAttr = FileAttr {
    ino: 4,
    size: 15,
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
    padding: 0,
};


struct HelloFS;

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else if parent == 1 && name.to_str() == Some("hello2.txt") {
            reply.entry(&TTL, &HELLO2_TXT_ATTR, 0);
        } else if parent == 1 && name.to_str() == Some("empty_dir") {
            reply.entry(&TTL, &EMPTY_DIR_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            4 => reply.attr(&TTL, &HELLO2_TXT_ATTR),
            3 => reply.attr(&TTL, &EMPTY_DIR_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else if ino == 4 {
            reply.data(&HELLO2_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {

        match ino {
            1 => {
                let entries = vec![
                    (1, FileType::Directory, "."),
                    (1, FileType::Directory, ".."),
                    (2, FileType::RegularFile, "hello.txt"),
                    (4, FileType::RegularFile, "hello2.txt"),
                    (3, FileType::Directory, "empty_dir"),
                    (3, FileType::Directory, "empty_dir_alias"),
                    (3, FileType::Directory, "empty_dir_alias2"),
                ];    
        
                for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
                    // i + 1 means the index of the next entry
                    if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                        break;
                    }
                }
        
            },
            3 => {
                let entries = vec![
                    (3, FileType::Directory, "."),
                    (1, FileType::Directory, ".."),
                ];

                for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
                    // i + 1 means the index of the next entry
                    if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                        break;
                    }
                }
            },
            _ => {
                reply.error(ENOENT);
                return;
            }
        }
        
        reply.ok();
    }
}

fn main() {
    let matches = Command::new("hello")
        .author("Christopher Berner")
        .arg(
            Arg::new("MOUNT_POINT")
                .required(true)
                .index(1)
                .help("Act as a client, and mount FUSE at given path"),
        )
        .arg(
            Arg::new("auto_unmount")
                .long("auto_unmount")
                .help("Automatically unmount on process exit"),
        )
        .arg(
            Arg::new("allow-root")
                .long("allow-root")
                .help("Allow root user to access filesystem"),
        )
        .get_matches();
    env_logger::init();
    let mountpoint = matches.value_of("MOUNT_POINT").unwrap();
    let mut options = vec![MountOption::RO, MountOption::FSName("hello".to_string())];
    if matches.is_present("auto_unmount") {
        options.push(MountOption::AutoUnmount);
    }
    if matches.is_present("allow-root") {
        options.push(MountOption::AllowRoot);
    }
    fuser::mount2(HelloFS, mountpoint, &options).unwrap();
}
