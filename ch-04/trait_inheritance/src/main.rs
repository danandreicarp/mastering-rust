struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn new(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date,
        }
    }
}

// simple trait
trait Vehicle {
    fn get_price(&self) -> u64;
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

// inheritance trait
trait Car: Vehicle {
    fn model(&self) -> String;
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

fn main() {
    let my_roadster = TeslaRoadster::new("Tesla Roadster II", 2020);
    println!(
        "{} is priced at ${}",
        my_roadster.model,
        my_roadster.get_price()
    );
}

// simple trait
trait Foo {
    fn foo();
}

// generic trait
trait FFoo<T> {
    fn ffoo(t: T) -> Self;
}

// associated type traits
struct Out;

trait AFoo {
    type Out;
    fn get_value(self) -> Self::Out;
}
