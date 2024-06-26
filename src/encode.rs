use std::io::{Cursor, Read};
use image::{RgbaImage, Rgba, ImageFormat};

pub fn encode(width: u32, input: &Vec<u8>) -> Vec<u8> {
    let height = calculate_image_height(width, input);

    let mut img = RgbaImage::new(width, height);

    let mut x = 0;
    let mut y = 0;

    for byte_chunk in input.chunks_exact(4) {

        let color = Rgba([byte_chunk[0], byte_chunk[1], byte_chunk[2], byte_chunk[3]]);
        x += 1;

        if x >= width {
            x = 0;
            y += 1;
        }

        img.put_pixel(x, y, color);
    }

    let mut final_byte = [0, 0, 0, 0];

    let remainder = input.chunks_exact(4).remainder();

    if remainder.len() > 0 {
        final_byte[0] = remainder[0];
    }

    if remainder.len() > 1 {
        final_byte[1] = remainder[1];
    }

    if remainder.len() > 2 {
        final_byte[2] = remainder[2];
    }

    img.put_pixel(x, y, Rgba(final_byte));

    let mut cursor = Cursor::new(Vec::new());
    let _ = img.write_to(&mut cursor, ImageFormat::Png);

    cursor.set_position(0);
    let mut buf: Vec<u8> = Vec::new();
    let _ = cursor.read_to_end(&mut buf);

    buf
}

fn calculate_image_height(width: u32, data: &Vec<u8>) -> u32 {
    let size = u32::try_from(data.len()).unwrap();
    (size / width) + 1
}

