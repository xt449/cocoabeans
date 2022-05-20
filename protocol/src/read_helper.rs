use std::io::Read;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub fn read_varint(read: &mut dyn Read) -> i32 {
    let mut value: i32 = 0;
    let mut position: i32 = 0;

    loop {
        let current_byte = read_unsigned_byte(read);
        value |= ((current_byte & SEGMENT_BITS) as i32) << position;

        if (current_byte & CONTINUE_BIT) == 0 {
            return value;
        }

        position += 7;

        if position >= 32 {
            // Too big
            return value;
        }
    }
}

pub fn read_unsigned_byte(read: &mut dyn Read) -> u8 {
    let byte_in: &mut [u8] = &mut [0];
    read.read_exact(byte_in).unwrap();
    return byte_in[0];
}
