use clap::{Parser, Subcommand};
use std::io::{self, Read, Write};
use std::fs;

mod encode;
mod decode;

const PIXEL_HEIGHT: u32 = 1;
const PIXEL_WIDTH: u32 = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Modes,

    #[arg(short, long)]
    input_file: Option<String>,

    #[arg(short, long)]
    output_file: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Modes {
    Encode {
        #[arg(short, long, default_value_t = 128)]
        width: u32,
    },
    Decode,
}

pub fn main() {
    let args = Args::parse();

    let mut input: Vec<u8> = Vec::new();

    match &args.input_file {
        Some(file_name) => {
            input = fs::read(file_name).unwrap();
        },
        None => {
            let _ = io::stdin().read_to_end(&mut input);
        }
    }

    let output: Vec<u8>;

    match &args.mode {
        Modes::Encode { width } => {
            output = encode::encode(*width, &input); 
        },
        Modes::Decode => {
            output = decode::decode(&input);
        },
    }

    match &args.output_file {
        Some(file_name) => {
            let _ = fs::write(file_name, output);
        },
        None => {
            let _ = io::stdout().write(&output);
        },
    }

//    let _ = img.save(output_file);
}

