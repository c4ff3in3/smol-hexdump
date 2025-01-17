use std::ffi::OsStr;
use std::fs;

fn get_file_contents(f: &OsStr) {
    let contents = match fs::read(f) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file contents: {e} ");
            return ();
        }
    };

    let mut iter = contents.chunks(2);

    let mut counter = 0;

    while let Some(chunk) = iter.next() {
        match chunk.len() {
            2 => print!("{:02X}{:02X} ", chunk[1], chunk[0]), // Combine two bytes
            1 => print!("{:02X} ", chunk[0]), // Handle the last byte if the length is odd
            _ => (),
        }
        counter += 1;

        if counter == 8 {
            counter = 0;
            println!();
        }
    }
}

pub fn generate_hex(f: &OsStr) {
    get_file_contents(f);
}
