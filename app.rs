use std::ffi::CString;
use std::ptr;
use simplefs_rs::simplefs; // Import your simplefs Rust module

fn main() {
    let ret: i32;
    let mut fd1: i32;
    let mut fd2: i32;
    let mut fd: i32;
    let mut i: i32;
    let mut c: u8;
    let mut buffer = [0u8; 1024];
    let buffer2: [u8; 8] = [50, 50, 50, 50, 50, 50, 50, 50];
    let mut size: i32;
    let mut vdiskname = [0u8; 200];

    println!("started");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: app <vdiskname>");
        std::process::exit(0);
    }
    let vdisk_arg = &args[1];
    let vdisk_cstring = CString::new(vdisk_arg.clone()).expect("CString::new failed");
    let vdisk_ptr = vdisk_cstring.as_ptr();
    unsafe {
        libc::strcpy(vdiskname.as_mut_ptr() as *mut i8, vdisk_ptr as *const i8);
    }

    ret = simplefs::sfs_mount(vdiskname.as_mut_ptr() as *mut i8);
    if ret != 0 {
        println!("could not mount");
        std::process::exit(1);
    }

    println!("creating files");
    simplefs::sfs_create(CString::new("file1.bin").unwrap().as_ptr() as *const i8);
    simplefs::sfs_create(CString::new("file2.bin").unwrap().as_ptr() as *const i8);
    simplefs::sfs_create(CString::new("file3.bin").unwrap().as_ptr() as *const i8);

    fd1 = simplefs::sfs_open(CString::new("file1.bin").unwrap().as_ptr() as *const i8, simplefs::MODE_APPEND);
    fd2 = simplefs::sfs_open(CString::new("file2.bin").unwrap().as_ptr() as *const i8, simplefs::MODE_APPEND);
    for i in 0..10000 {
        buffer[0] = 65;
        simplefs::sfs_append(fd1, buffer.as_mut_ptr() as *mut libc::c_void, 1);
    }

    for i in 0..10000 {
        buffer[0] = 65;
        buffer[1] = 66;
        buffer[2] = 67;
        buffer[3] = 68;
        simplefs::sfs_append(fd2, buffer.as_mut_ptr() as *mut libc::c_void, 4);
    }

    simplefs::sfs_close(fd1);
    simplefs::sfs_close(fd2);

    fd = simplefs::sfs_open(CString::new("file3.bin").unwrap().as_ptr() as *const i8, simplefs::MODE_APPEND);
    for i in 0..10000 {
        libc::memcpy(buffer.as_mut_ptr() as *mut libc::c_void, buffer2.as_ptr() as *const libc::c_void, 8);
        simplefs::sfs_append(fd, buffer.as_mut_ptr() as *mut libc::c_void, 8);
    }
    simplefs::sfs_close(fd);

    fd = simplefs::sfs_open(CString::new("file3.bin").unwrap().as_ptr() as *const i8, simplefs::MODE_READ);
    size = simplefs::sfs_getsize(fd);
    for i in 0..size {
        simplefs::sfs_read(fd, buffer.as_mut_ptr() as *mut libc::c_void, 1);
        c = buffer[0];
        c = c.wrapping_add(1);
    }
    simplefs::sfs_delete(CString::new("file3.bin").unwrap().as_ptr() as *const i8);
    simplefs::sfs_close(fd);
    ret = simplefs::sfs_umount();
}
