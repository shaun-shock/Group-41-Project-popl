const DIR_ENTRY_SIZE: usize = 110;
const BITMAP_SIZE: usize = 4096;
const BIT_OFFSET: usize = 8;

struct DIR {
    name: [char; DIR_ENTRY_SIZE],
    index: i32,
}

struct FCB {
    available: i32,
    index_block: i32,
    size: i32,
}

struct FILES {
    free: i32,
    name: [char; DIR_ENTRY_SIZE],
    index: i32,
    mode: i32,
    offset: i32,
    size: i32,
    total: i32,
}

fn set_bit(bitmap: &mut [u8], nblock: usize, bit_index: u32) {
    let curr = &mut bitmap[BITMAP_SIZE * nblock..];
    let char_index = (bit_index / BIT_OFFSET) as usize;
    let index = (BIT_OFFSET - 1) - (bit_index % BIT_OFFSET);
    let curr = &mut curr[char_index * std::mem::size_of::<u8>()..];
    curr[0] |= 1u8 << index;
}

fn get_bit(bitmap: &[u8], nblock: usize, bit_index: u32) -> i32 {
    let curr = &bitmap[BITMAP_SIZE * nblock..];
    let char_index = (bit_index / BIT_OFFSET) as usize;
    let index = 7 - (bit_index % BIT_OFFSET);
    let curr = &curr[char_index * std::mem::size_of::<u8>()..];
    if (curr[0] & (1u8 << index)) == 0 {
        0
    } else {
        1
    }
}

fn clear_bit(bitmap: &mut [u8], nblock: usize, bit_index: u32) {
    let curr = &mut bitmap[BITMAP_SIZE * nblock..];
    let char_index = (bit_index / BIT_OFFSET) as usize;
    let index = 7 - (bit_index % BIT_OFFSET);
    let curr = &mut curr[char_index * std::mem::size_of::<u8>()..];
    curr[0] &= !(1u8 << index);
}

