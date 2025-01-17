mod hex;

use std::env;

fn main() {
    if env::args().len() == 1 {
        println!("No Args");
        return;
    }

    let file_args: Vec<String> = env::args().skip(1).collect();

    let cwd = env::current_dir().unwrap();

    let files_to_dump = file_args.iter().map(|f| {
        let mut pth = cwd.clone().into_os_string();
        pth.push(format!("/{f}"));
        pth
    });

    for f in files_to_dump {
        hex::generate_hex(&f);
    }
}
