use std::time::SystemTime;

use fuser::{FileAttr, FileType};

pub struct File {
    // id: u64,
    name: String,
    file_attr: FileAttr,
}

// impl File {
//     fn create(name: String, content: String) -> File {
//         let now = SystemTime::now();
//         let file_attr: FileAttr = FileAttr {
//             ino: 2,
//             size: 13,
//             blocks: 1,
//             atime: now, // 1970-01-01 00:00:00
//             mtime: now,
//             ctime: now,
//             crtime: now,
//             kind: FileType::RegularFile,
//             perm: 0o644,
//             nlink: 1,
//             uid: 501,
//             gid: 20,
//             rdev: 0,
//             flags: 0,
//             blksize: 512,
//             padding: 0,
//         };

//         File {}
//     }
// }

pub struct Directory {
    id: u64,
    name: String,
    file_attr: FileAttr,
}
