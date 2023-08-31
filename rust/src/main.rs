use std::io;
use rand::Rng;
use regex::Regex;

fn main() {
    let re: Regex = Regex::new(r"^\d+$").unwrap();

    let mut passwd: String = String::new();
    let mut input:  String = String::new();

    println!("Password length (leave blank for default):");
    io::stdin().read_line(&mut input).expect("Unexpected error");

    let length: i32 = if re.is_match(input.trim()) {
        input.trim().parse().unwrap()
    } else {
        16
    };

    for _ in 0..length {
        let n: u8 = rand::thread_rng().gen_range(33..127);
        passwd.push(char::from(n));
    }

    println!("{}", passwd);
}
