use std::ffi::CString;
use std::io::Read;

fn main() {
    let mut buf = [0u8; 4];
    let _ = getrandom::getrandom(&mut buf).unwrap();
    let number = u32::from_le_bytes(buf);
    let string = CString::new(format!("=={:08x}==", number)).unwrap();

    println!("Press any key to reveal the secret...");
    std::io::stdin().read_exact(&mut [0u8]).unwrap();

    println!("The secret is: {:?}", string);
}
