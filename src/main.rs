use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No arguments provided.");
        return;
    }

    if args.len() > 2 {
        eprintln!("Only provide one argument.");
        return;
    }

    let password_length : usize = args[1].parse().expect("The password could not be converted into an usize. Please provide a proper value.");

    println!("{}", generate_password(password_length));
}


fn generate_password(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut password = String::with_capacity(length);
    let mut rng = rand::rng();

    for _ in 0..length {
        let idx = rng.random_range(0..charset.len());
        password.push(charset.chars().nth(idx).expect("Something went wrong while generating the password."));
    }

    password
}
