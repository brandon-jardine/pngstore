use clap::{Args, Parser, Subcommand};
use std::io::{self, Read, Write};
use std::fs;

mod encode;
mod decode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version(true))]
struct Cli {
    #[command(subcommand)]
    mode: Modes,
}

#[derive(Args, Debug)]
struct FileArgs {
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

        #[command(flatten)]
        file_args: FileArgs,
    },
    Decode {
        #[command(flatten)]
        file_args: FileArgs,
    },
}

pub fn main() {
    let args = Cli::parse();

    match &args.mode {
        Modes::Encode { width, file_args } => {
            let output = encode::encode(*width, &load_input(&file_args.input_file)); 
            write_output(&file_args.output_file, &output);
        },
        Modes::Decode { file_args } => {
            let output = decode::decode(&load_input(&file_args.input_file));
            write_output(&file_args.output_file, &output);
        },
    }
}

fn load_input(file_name: &Option<String>) -> Vec<u8> {
    match &file_name {
        Some(file_name) => {
            fs::read(file_name).unwrap()
        },
        None => {
            let mut contents = Vec::new();
            let _ = io::stdin().read_to_end(&mut contents);
            contents
        }
    }
}

fn write_output(file_name: &Option<String>, contents: &Vec<u8>) {
    match &file_name {
        Some(name) => {
            let _ = fs::write(name, contents).unwrap();
        },
        None => {
            let _ = io::stdout().write(&contents).unwrap();
        }
    }
}

