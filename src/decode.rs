use image::io::Reader as ImageReader;
use std::io::Cursor;
use image::Pixel;

pub fn decode(input: &Vec<u8>) -> Vec<u8> {
    // let img2 = ImageReader::open(input_file).unwrap().decode().unwrap();
    let img2 = ImageReader::new(Cursor::new(input)).with_guessed_format().unwrap().decode().unwrap();

    let mut text_bytes: Vec<u8> = Vec::new();

    for pixel in img2.to_rgba8().pixels().map(|p| p.to_rgba()) {
        text_bytes.extend_from_slice(pixel.channels());
    }

    text_bytes
}

