#![allow(dead_code)]

use std::str;

fn give_me<T>(value: T) {
    let _ = value;
}

struct GenericStruct<T>(T);

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

// impl Container<u32> {
//     fn new(item: u32) -> Self {
//         Container { item }
//     }
// }

enum Transmission<T> {
    Signal(T),
    NoSignal,
}

fn main() {
    let a = "generics";
    let b = 1024;
    give_me(a);
    give_me(b);

    let _tea = Container::new("Tea");

    // using generics

    let _v1: Vec<u8> = Vec::new();
    let mut v2 = Vec::new();
    v2.push(2);
    let _v3 = Vec::<u8>::new(); // turbofish

    // let num_from_str: u8 = str::parse("34").unwrap();
    let num_from_str = str::parse::<u16>("34").unwrap();
    println!("Parsed number {}", num_from_str);
}
