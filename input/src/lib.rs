use std::{fs, io, path::PathBuf};

fn stdin_path() -> PathBuf {
    println!();
    println!("enter path to input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin io error");
    PathBuf::from(input.trim())
}

pub fn stdin_file() -> String {
    loop {
        let path = stdin_path();
        println!(
            "reading: {}",
            path.canonicalize().as_ref().unwrap_or(&path).display()
        );
        match fs::read_to_string(path) {
            Ok(s) => return s,
            Err(e) => eprintln!("file read io error: {e}"),
        }
    }
}
