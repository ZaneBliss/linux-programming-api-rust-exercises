use std::{env, fs::File, io::{self}, process};

fn main() -> io::Result<()> {
   let (from_f, to_f) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(from), Some(to)) => (File::open(from), File::create(to)),
        _ => {
            eprintln!("Process exited with code {}", 1);
            process::exit(1);
        }
    };

   io::copy(&mut from_f?, &mut to_f?)?;

    Ok(())
}
