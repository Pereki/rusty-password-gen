use std::env;
use std::process;
use rand::Rng;

fn main() {
    
    let mut args = env::args();
    args.next();

    let len_str = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("No arguments provided.");
            process::exit(1);
        }
    };

    let password_length : usize = match len_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid length provided.");
            process::exit(1);
        }
    };

    println!("{}", generate_password(password_length));
}


fn generate_password(length: usize) -> String {
    let charset: [char; 62] = [
        'A','B','C','D','E','F','G','H','I','J','K','L','M',
        'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        'a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z',
        '0','1','2','3','4','5','6','7','8','9'
    ];
    let mut password = String::with_capacity(length);
    let mut rng = rand::rng();


    for _ in 0..length {
        let idx: usize = rng.random_range(0..charset.len());
        password.push(charset[idx]);
    }

    password
}
