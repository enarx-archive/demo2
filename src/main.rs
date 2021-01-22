use std::{thread, time};
use std::ffi::CString;
//use std::io::Read;

fn main() {
    let wait_secs = 20;
    let mut buf = [0u8;4];
    let _ = getrandom::getrandom(&mut buf).unwrap();
    let number = u32::from_le_bytes(buf);
    let string = CString::new(format!("=={:08x}==", number)).unwrap();
    println!("Secret created - wait {} seconds!", wait_secs);
    let wait_time = time::Duration::from_secs(wait_secs);
    let now = time::Instant::now();
    thread::sleep(wait_time);
    assert!(now.elapsed() >= wait_time);
    //println!("Press ENTER to reveal the secret...");
    //std::io::stdin().read_exact(&mut [0u8]).unwrap();

    println!("The secret is: {:?}", string);
}
