use std::process::Command;

use lab_2_class_construction::{input_dimension, utils::cin, Dimensions};

fn main() {
    let sentinel = 'N';

    let mut room = Dimensions::default();

    let mut gallons: f64;

    let mut sentinel_store = 'U';

    while sentinel_store != sentinel {
        Command::new("clear").status().unwrap();

        room.length = input_dimension("Please provide a length", None);
        room.width = input_dimension("Please provide a width", None);
        room.height = input_dimension("Please provide a height", None);

        gallons = room.calc_volume();

        println!("Gallons required: {}", gallons);
        println!("Would you like to calculate for another room? Type 'N' if not");

        sentinel_store = cin().pop().unwrap();
    }

    println!("Thanks!");
}
