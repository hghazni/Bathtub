use std::io;

fn main() {
    println!("Why hello there my good man. What can I measure in bathtub for you today?");

    // variables
    let mut measurement = String::new();
    let bathtubs = String::from("bathtubs.");

    io::stdin().read_line(&mut measurement)
        .expect("Error getting bathtubs");

    println!("You're measurement is: {:?}, {:?}", measurement.trim(), bathtubs);

}
