use std::collections::HashMap;

fn main() {
    // variables
    let mut target = "world";
    let mut greeting = "Hello";
    println!("{}, {}", greeting, target);
    greeting = "How are you doing";
    target = "mate";
    println!("{}, {}", greeting, target);

    //functions
    let a: u64 = 17;
    let b = 3;
    let result = add(a, b);
    println!("Result {}", result);

    // mutable functions
    let score = 2048;
    increase_by(score, 30);

    // closures
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!("{} doubled is {}", value, twice);

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };

    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);

    // strings
    let question = "How are you?";
    let person: String = "Bob".to_string();
    let namaste = String::from("नमस्ते");

    println!("{}! {} {}", namaste, question, person);

    // if else
    let rust_is_awesome = true;
    if rust_is_awesome {
        println!("Indeed");
    } else {
        println!("Well, you should try Rust!");
    }

    // if else assign
    let result = if 1 == 2 {
        "Wait, what?"
    } else {
        "Rust makes sense"
    };
    println!("You know what? {}", result);

    // if else no value
    let result = if 1 == 2 {
        "Nothing makes sense";
    } else {
        "Sanity reigns";
    };
    println!("Result of computation: {:?}", result);

    // match expressions
    let status = req_status();
    match status {
        200 => println!("Success"),
        404 => println!("Not Found"),
        other => {
            println!("Request failed with code {}", other);
            // get response from cache
        }
    }

    // loops
    let mut x = 8;
    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go", x);
        x -= 1;
    }

    // loop labels
    let a = 10;
    let b = 4;
    let result = silly_sub(a, b);
    println!("{} minus {} is {}", a, b, result);

    // while
    let mut x = 8;
    while x > 0 {
        println!("{} more runs to go", x);
        x -= 1;
    }

    // for loops
    // does not include 10
    print!("Normal ranges: ");
    for i in 0..10 {
        print!("{},", i);
    }

    println!(); // just a newline
    print!("Inclusive ranges: ");
    for i in 0..=10 {
        print!("{},", i);
    }
    println!(); // just a newline

    // unit struct
    let _value = Dummy;

    // tuple struct
    let white = Color(255, 255, 255);

    // You can pull them out by index
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);

    let orange = Color(255, 165, 0);

    // You can also destructure the fields directly
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    // Can also ignore fields while destructuring
    let Color(r, _, b) = orange;
    println!("R: {} and B: {} in Orange", r, b);

    // structs
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };
    bump_player_score(player, 120);

    // enums
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };

    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {}",
                direction, speed
            )
        }
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    }

    // impl blocks on structs
    let mut player = Player::with_name("Dave");
    player.set_friends(23);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    // another way to call instance methods
    let _ = Player::get_friends(&player);

    // impl blocks for enums
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);

    // arrays
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.1f64, 0.2, 0.3];
    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);

    // tuples
    let num_and_str: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);

    // vectors
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![1];
    vec_with_macro.push(2);
    let _ = vec_with_macro.pop();

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);

    // hashmaps
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\nI now have {} avocados", fruits["avocado"]);

    // slices
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }

    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}

fn req_status() -> u32 {
    200
}

fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            'decrement: loop {
                if dec == 0 {
                    // breaks directly out of 'increment loop
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }

    result
}

struct Dummy;

struct Color(u8, u8, u8);

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += score;
    println!("Updated payer stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
            score: 0,
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

fn pay_by_credit(amount: u64) {
    println!("Processing credit payment of {}", amount);
}

fn pay_by_debit(amount: u64) {
    println!("Processing debit payment of {}", amount);
}

fn paypal_redirect(amount: u64) {
    println!("Redirecting to paypal for amount: {}", amount);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}
