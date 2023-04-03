// practice with modules
mod bar;
pub use self::bar::Bar;

pub fn do_foo() {
    println!("Hello from Foo!");
}
