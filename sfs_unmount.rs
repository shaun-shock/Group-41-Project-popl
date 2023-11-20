use std::fs::File;
use std::io;
use std::io::prelude::*;

static mut VDISK_FD: i32 = -1;

fn sfs_umount() -> io::Result<()> {
    unsafe {
        if VDISK_FD != -1 {
            // fsync
            let vdisk = File::from_raw_fd(VDISK_FD);
            vdisk.sync_all()?;

            // close
            drop(vdisk);
            VDISK_FD = -1;
        }
    }
    Ok(())
}
