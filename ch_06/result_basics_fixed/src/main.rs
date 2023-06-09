use std::io::Read;
use std::{fs::File, path::Path};

fn main() {
    let path = Path::new("data.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Error while opening file: {}", err),
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    println!("Message: {}", s);
}
