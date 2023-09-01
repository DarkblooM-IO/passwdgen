// thanks to orhun for their article on random number generation
// -> https://blog.orhun.dev/zero-deps-random-in-rust/

use std::io;
use std::fs::File;
use std::io::{Read, Result};

fn rand(target: &mut u8, min: u8, max: u8) -> Result<()> {
    let mut rng    = File::open("/dev/urandom")?;
    let mut buffer = [0u8; 1];

    let _ = rng.read_exact(&mut buffer);
    
    *target = (buffer[0] % (max - min + 1)) + min;

    Ok(())
}

fn main() {
    let length: u8;

    let mut password: String = String::from("");
    let mut input:    String = String::new();

    println!("Password length (leave blank for default):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get length");

    length = match input.trim().parse::<u8>() {
        Ok(i) => if i > 0 { i } else { 16 },
        Err(..) => 16,
    };

    for _ in 0..length {
        let mut n: u8 = 0;
        let _ = rand(&mut n, 33, 126);
        password.push(n as char);
    }

    println!("{}", password);
}
