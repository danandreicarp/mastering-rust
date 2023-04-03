// practice with modules
#[allow(dead_code)]
mod food {
    #[derive(Debug)]
    pub struct Broccoli;
    struct Smoothie;
    struct Hummus;
}

use food::Broccoli;

mod foo;
use foo::Bar;

// imgtool
use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);

    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    let mut new_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
    new_file_name.push_str("_rotated");

    rotated
        .save(
            path.with_file_name(new_file_name)
                .with_extension(path.extension().unwrap()),
        )
        .unwrap();

    // practice with modules
    let eatable = Broccoli;
    println!("Hello, world! I'm having a {:?}!", eatable);

    foo::do_foo();
    Bar::hello();
}
