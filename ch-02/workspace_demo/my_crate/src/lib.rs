pub fn greet() {
    println!("Hi from my_crate!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
