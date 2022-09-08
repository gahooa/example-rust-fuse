use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};
use libc::ENOENT;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
mod appcove;
use glob::glob;

use log::{info, warn};

const TTL: Duration = Duration::from_secs(2); // 1 second

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

const APPCOVE_TMP_FILE_ATTR: FileAttr = FileAttr {
    ino: 1,
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

struct AppCoveFSdata {
    data: HashMap<u64, String>,
}

impl AppCoveFSdata {
    fn new() -> AppCoveFSdata {
        AppCoveFSdata {
            data: HashMap::new(),
        }
    }
    fn mkdir(name: String) {}
    fn touch(name: String) {}
}

pub struct AppCoveFS {
    body: AppCoveFSdata,
}

impl AppCoveFS {
    fn new() -> AppCoveFS {
        AppCoveFS {
            body: AppCoveFSdata::new(),
        }
    }
    pub fn launch(args: &super::cli::Args) -> std::io::Result<()> {
        // if option selection becomes more sofisticated i suggest https://crates.io/crates/derive_builder
        let mut options = vec![MountOption::RO, MountOption::FSName("hello".to_string())];
        if args.auto_unmount {
            options.push(MountOption::AutoUnmount);
        }
        if args.allow_root {
            options.push(MountOption::AllowRoot);
        }

        fuser::mount2(AppCoveFS::new(), &args.mount_point, &options)
    }
}

impl Filesystem for AppCoveFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        // if parent == 1 && name.to_str() == Some("hello.txt") {
        //     reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        // } else if parent == 1 && name.to_str() == Some("hello2.txt") {
        //     reply.entry(&TTL, &HELLO2_TXT_ATTR, 0);
        // } else {
        //     reply.error(ENOENT);
        // }

        dbg!(parent, name);
        // println!();

        let banned_files = [
            ".Trash",
            ".Trash-1000",
            ".xdg-volume-info",
            "autorun.inf",
            ".git",
            ".gitignore",
            "HEAD",
        ];

        let to_search_name = name.to_str().unwrap();
        // if !banned_files.contains(&to_search_name) {
        //     println!("i want {to_search_name}");

        // match (
        //     banned_files.contains(&to_search_name),
        //     glob(&format!("*/{to_search_name}")),
        // ) {
        //     (false, Ok(paths)) => {
        //         let combined_content = paths
        //             .filter_map(Result::ok)
        //             .map(|entry| std::fs::read_to_string(&entry).unwrap())
        //             .collect::<String>();
        //         self.body.data.insert(10, combined_content);
        //         reply.entry(&TTL, &APPCOVE_TMP_FILE_ATTR, 0);
        //         info!("inside lookup");
        //     }
        //     _ => reply.error(ENOENT),
        // }

        //     // let combined_content = glob(&format!("*/{to_search_name}"))
        //     //     .expect("Failed to read glob pattern")
        //     //     .filter_map(Result::ok)
        //     //     .map(|entry| std::fs::read_to_string(&entry).unwrap())
        //     //     .collect::<String>();
        //     self.body.data.insert(10, combined_content);
        //     reply.entry(&TTL, &APPCOVE_TMP_FILE_ATTR, 0);
        // } else {
        //     reply.error(ENOENT);
        // }
        // // } else {
        // //     reply.error(ENOENT);
        // }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let appcove_file = FileAttr {
            ino: 10,
            size: 0,
            blocks: 0,
            atime: UNIX_EPOCH, // 1970-01-01 00:00:00
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o755,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: 512,
            padding: 0,
        };
        info!("inside getattr");

        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            4 => reply.attr(&TTL, &HELLO2_TXT_ATTR),
            10 => reply.attr(&TTL, &appcove_file),
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
        info!("inside read");
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else if ino == 4 {
            reply.data(&HELLO2_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else if ino == 10 {
            reply.data(&self.body.data.get(&10).unwrap().as_bytes()[offset as usize..]);
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
        info!("inside read");
        match ino {
            1 => {
                let entries = vec![
                    (1, FileType::Directory, "."),
                    (1, FileType::Directory, ".."),
                    (2, FileType::RegularFile, "hello.txt"),
                    (4, FileType::RegularFile, "hello2.txt"),
                ];

                for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
                    // i + 1 means the index of the next entry
                    if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                        break;
                    }
                }
            }
            _ => {
                reply.error(ENOENT);
                return;
            }
        }

        reply.ok();
    }
}
