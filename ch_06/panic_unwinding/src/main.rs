use std::{panic, thread};

fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}

fn bob() {
    malice();
}

fn malice() {
    panic!("malice is panicking!");
}

fn main() {
    let child = alice();
    let _ = child.join();

    panic::catch_unwind(|| {
        panic!("Panicking!");
    })
    .ok();
    println!("Survived that panic");

    bob();
    println!("This is unreacheable code");
}
