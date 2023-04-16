#![allow(dead_code)]

struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

// trait_bound_basics
use std::ops::Add;

fn add_things<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);

    // trait_bound_basics
    add_things(2, 2);

    // trait bounds on functions
    let apple = Food(Apple);
    eat(apple);

    // traits composition
    Bob.animate();

    // trait bounds with impl trait syntax
    show_me("Trait bounds are awesome!");

    // impl trait clojure
    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());

    // impl trait both
    println!("{}", surround_with_braces("Hello"));
}

// trait bounds on types
use std::fmt::Display;

struct Foo<T: Display> {
    bar: T,
}

// or

struct Bar<F>
where
    F: Display,
{
    inner: F,
}

// trait bounds on functions

use std::fmt::Debug;

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

impl<T> Eatable for Food<T>
where
    T: Debug,
{
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

fn eat<T>(val: T)
where
    T: Eatable,
{
    val.eat();
}

// traits composition
trait Eat {
    fn eat(&self) {
        println!("eat")
    }
}

trait Code {
    fn code(&self) {
        println!("code")
    }
}

trait Sleep {
    fn sleep(&self) {
        println!("sleep")
    }
}

trait Programmer: Eat + Code + Sleep {
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
        println!("repeat");
    }
}

struct Bob;
impl Programmer for Bob {}
impl Eat for Bob {}
impl Code for Bob {}
impl Sleep for Bob {}

// trait bounds with impl trait syntax
fn show_me(val: impl Display) {
    println!("{}", val);
}

// impl trait clojure
fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

// impl trait both
fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}
