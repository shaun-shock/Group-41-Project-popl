use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::mem;
use std::ptr;

const MODE_READ: i32 = 0;
const MODE_APPEND: i32 = 1;
const BLOCKSIZE: usize = 4096;

const DIR_ENTRY_SIZE: usize = 110;
const BITMAP_SIZE: usize = 4096;
const BIT_OFFSET: usize = 8;
const MAX_FILES: usize = 128;
const MAX_OPENED_FILES: usize = 16;
const MAX_OPENED_BITS: usize = 15;
const BITS: usize = 13;
const SUPERBLOCK_START: usize = 0;
const SUPERBLOCK_COUNT: usize = 1;
const BITMAP_START: usize = 1;
const DIR_START: usize = 5;
const DIR_COUNT: usize = 4;
const FCB_START: usize = 9;
const FCB_COUNT: usize = 4;
const FCB_SIZE: usize = 128;
const MAX_FCB: usize = 32768;
const MAX_FILE_SIZE: usize = 32768;
const DISK_POINTER_SIZE: usize = 4096;
const DIR_ENTRY_PER_BLOCK: usize = 32;
const MAX_BITS_SIZE: usize = 1024;
const MAX_BIT_SIZE_ACTUAL: usize = 1023;

struct DIR {
    name: [u8; DIR_ENTRY_SIZE],
    index: i32,
}

struct FCB {
    available: i32,
    index_block: i32,
    size: i32,
}

fn read_block(fd: &mut File, buffer: &mut [u8], block_num: usize) -> io::Result<()> {
    let offset = block_num * BLOCKSIZE;
    fd.seek(SeekFrom::Start(offset as u64))?;
    fd.read_exact(buffer)?;
    Ok(())
}

fn write_block(fd: &mut File, buffer: &[u8], block_num: usize) -> io::Result<()> {
    let offset = block_num * BLOCKSIZE;
    fd.seek(SeekFrom::Start(offset as u64))?;
    fd.write_all(buffer)?;
    Ok(())
}

fn get_bit(buffer: &[u8], block_num: usize, bit_num: usize) -> i32 {
    let byte_offset = bit_num / BIT_OFFSET;
    let bit_offset = bit_num % BIT_OFFSET;
    let byte = buffer[block_num * BLOCKSIZE + byte_offset];
    (byte >> (BIT_OFFSET - 1 - bit_offset)) & 1
}

fn set_bit(buffer: &mut [u8], block_num: usize, bit_num: usize) {
    let byte_offset = bit_num / BIT_OFFSET;
    let bit_offset = bit_num % BIT_OFFSET;
    buffer[block_num * BLOCKSIZE + byte_offset] |= 1 << (BIT_OFFSET - 1 - bit_offset);
}

fn sfs_create(filename: &str) -> io::Result<i32> {
    let mut vdisk = OpenOptions::new().read(true).write(true).open("your_vdisk_name.img")?;

    let mut traverse = BLOCKSIZE * DIR_START;
    let mut median = vec![0; SUPERBLOCK_COUNT * MAX_FILES];
    vdisk.seek(SeekFrom::Start(traverse as u64))?;
    vdisk.read_exact(&mut median)?;

    let mut size = 0;
    while size < MAX_FILES {
        let dir_entry: &mut DIR = unsafe { &mut *(median[size * mem::size_of::<DIR>()..].as_mut_ptr() as *mut _) };
        if dir_entry.index == -1 {
            let fcb_start = BLOCKSIZE * FCB_START;
            let mut median_two = vec![0; SUPERBLOCK_COUNT * FCB_SIZE];
            vdisk.seek(SeekFrom::Start(fcb_start as u64))?;
            vdisk.read_exact(&mut median_two)?;

            let mut size_two = 0;
            while size_two < FCB_SIZE {
                let fcb: &mut FCB =
                    unsafe { &mut *(median_two[size_two * mem::size_of::<FCB>()..].as_mut_ptr() as *mut _) };
                if fcb.available == 0 {
                    dir_entry.index = size_two;
                    fcb.available = 1;

                    let mut median_three = vec![0; SUPERBLOCK_COUNT * BLOCKSIZE];
                    for iterate_out in SUPERBLOCK_COUNT..FCB_COUNT {
                        read_block(&mut vdisk, &mut median_three, iterate_out)?;
                        for iterate in SUPERBLOCK_START..MAX_FCB {
                            if get_bit(&median_three, SUPERBLOCK_START, iterate) == SUPERBLOCK_START {
                                set_bit(&mut median_three, SUPERBLOCK_START, iterate);
                                write_block(&mut vdisk, &median_three, iterate_out)?;
                                fcb.index_block = iterate + ((iterate_out - SUPERBLOCK_COUNT) * (SUPERBLOCK_COUNT << MAX_OPENED_BITS));
                                fcb.size = SUPERBLOCK_START;

                                let mut index_buff = vec![0; SUPERBLOCK_COUNT * BLOCKSIZE];
                                let mut index_curr = index_buff.as_mut_ptr() as *mut i32;
                                for _ in 0..(BLOCKSIZE / FCB_COUNT) {
                                    unsafe {
                                        ptr::write(index_curr, -1);
                                        index_curr = index_curr.add(1);
                                    }
                                }

                                write_block(&mut vdisk, &index_buff, fcb.index_block)?;
                                traverse = FCB_START * BLOCKSIZE + FCB_SIZE * size_two;
                                vdisk.seek(SeekFrom::Start(traverse as u64))?;
                                vdisk.write_all(&median_two)?;

                                traverse = DIR_START * BLOCKSIZE + FCB_SIZE * size;
                                vdisk.seek(SeekFrom::Start(traverse as u64))?;
                                vdisk.write_all(&median)?;
                                return Ok(0);
                            }
                        }
                    }
                    return Ok(-1);
                }
                size_two += 1;
            }
            return Ok(-1);
        }
        size += 1;
    }
    Ok(-1)
}
