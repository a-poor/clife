// use std::error::Error;
use std::collections::HashMap;


#[allow(dead_code)]
struct Command {
    name: String,
    short: Vec<String>,
    run: fn() -> (),
}

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 6);
    scores.insert(String::from("Red"), 202);

    std::env::args().enumerate().for_each(|(i, arg)| {
        println!("{}: {}", i, arg);
    });
}
