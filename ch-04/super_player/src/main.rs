mod media;

use crate::media::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

fn main() {
    println!("Hello, world!");
}
