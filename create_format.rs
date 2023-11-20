use std::env;
use std::process;
use std::ffi::CString;
use std::os::raw::c_char;
use std::mem::MaybeUninit;

extern "C" {
    fn create_format_vdisk(vdiskname: *const c_char, m: i32) -> i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: create_format <vdiskname> <m>");
        process::exit(1);
    }
    let vdiskname = CString::new(args[1].clone()).expect("CString::new failed");
    let m = args[2].parse::<i32>().expect("Failed to parse <m>");

    println!("started");
    let ret = unsafe { create_format_vdisk(vdiskname.as_ptr(), m) };
    if ret != 0 {
        eprintln!("there was an error in creating the disk");
        process::exit(1);
    }
    println!("disk created and formatted. {} {}", args[1], m);
}

