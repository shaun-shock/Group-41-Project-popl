use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::io;
use std::io::prelude::*;

const MAX_OPENED_FILES: usize = 16;
const SUPERBLOCK_START: i32 = 0;
const DIR_ENTRY_SIZE: usize = 110;

struct FileEntry {
    free: i32,
    name: [u8; DIR_ENTRY_SIZE],
    index: i32,
    mode: i32,
    offset: i32,
    size: i32,
    total: i32,
}

const INIT_FILE_ENTRY: FileEntry = FileEntry {
    free: SUPERBLOCK_START,
    name: [0; DIR_ENTRY_SIZE],
    index: -1,
    mode: 0,
    offset: 0,
    size: SUPERBLOCK_START,
    total: 0,
};

static mut OFT: [FileEntry; MAX_OPENED_FILES] = [INIT_FILE_ENTRY; MAX_OPENED_FILES];
static mut VDISK_FD: i32 = -1;

fn sfs_mount(vdiskname: &str) -> io::Result<()> {
    unsafe {
        for entry in OFT.iter_mut() {
            *entry = INIT_FILE_ENTRY;
        }

        let vdisk = OpenOptions::new().read(true).write(true).open(vdiskname)?;
        VDISK_FD = vdisk.as_raw_fd();
    }
    Ok(())
}
