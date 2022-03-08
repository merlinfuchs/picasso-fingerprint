use quad_rand as qrand;
use numtoa::NumToA;
use std::str;

/*
We are re-using a byte-array for all dynamic strings to avoid heap allocations;
String operations bloat the bundle size and this is a lot faster anyways

The only thing to keep in mind is that all string slices point to the same underlying data;
Having more than one string can therefore lead to undefined behavior
*/

const STRING_BUFFER_SIZE: usize = 64;
static mut STRING_BUFFER: [u8; STRING_BUFFER_SIZE] = [0; STRING_BUFFER_SIZE];

pub fn get_random_color() -> &'static str {
    /*
    basically the same as
    format!("#{:X}", qrand::gen_range(0, 0xFFFFFF)
     */
    let mut buffer = unsafe { &mut STRING_BUFFER[..7] };

    buffer[0] = b'#';
    let value = qrand::gen_range(0, 0xFFFFFF);
    value.numtoa(16, &mut buffer[1..]);

    unsafe { str::from_utf8_unchecked(&STRING_BUFFER[..7]) }
}

pub fn get_random_string(length: usize) -> &'static str {
    let mut buffer = unsafe { &mut STRING_BUFFER[..length] };
    for i in 0..buffer.len() {
        buffer[i] = qrand::gen_range(b'0', b'z');
    }

    unsafe { str::from_utf8_unchecked(&STRING_BUFFER[..length]) }
}

pub fn format_pixel_value(pixel: u32) -> &'static str {
    let mut buffer = unsafe { &mut STRING_BUFFER };

    buffer[STRING_BUFFER_SIZE - 2] = b'p';
    buffer[STRING_BUFFER_SIZE - 1] = b'x';

    let offset = pixel.numtoa(10, &mut buffer[..STRING_BUFFER_SIZE - 2]).len();

    unsafe { str::from_utf8_unchecked(&STRING_BUFFER[64 - offset - 2..]) }
}
