use rand::prelude::*;
use std::io::Read;

fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen();
    let string = format!("{:08x}", number);

    println!("Press any key to reveal the secret...");
    std::io::stdin().read_exact(&mut [0u8]).unwrap();

    println!("The secret is: {}", string);
}
