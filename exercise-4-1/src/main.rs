use std::{env::args, fs::{File, OpenOptions}, io::{self, Write}, process};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let file_name = match args().nth(1) {
        Some(name) => name,
        None => { 
            eprintln!("Please provide a filename.");
            process::exit(1);
        }
    };

    let append_flag = match args().nth(2) {
        Some(arg) => if arg == "-a" { "true" } else { "false" },
        None => "false",
    };

    let buffer;
    if append_flag.parse().unwrap() {
        buffer = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
    } else {
        buffer = Ok(File::create(file_name)?);
    }

    io::stdin().read_line(&mut input)?;

    match buffer?.write_all(input.as_bytes()) {
        Ok(..) => {
            print!("{input}")
        },
        Err(..) => {
            std::process::exit(-1)
        }
    }

    Ok(())
}
