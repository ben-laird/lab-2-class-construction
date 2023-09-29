use std::io::{self, BufRead};

pub fn cin() -> String {
    io::stdin().lock().lines().next().unwrap().unwrap()
}

pub fn cout(message: &str) {
    println!("{}", message);
}
