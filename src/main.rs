#![feature(wasi_ext)]

use std::ffi::CString;
use std::io::prelude::*;
use std::os::wasi::prelude::*;

fn main() {
    let mut buf = [0u8; 4];
    let _ = getrandom::getrandom(&mut buf).unwrap();
    let number = u32::from_le_bytes(buf);
    let string = CString::new(format!("=={:08x}==", number)).unwrap();

    let socket_fd = unsafe { wasi_socket::accept().unwrap() };
    let mut socket = unsafe { std::fs::File::from_raw_fd(socket_fd) };

    socket.write_all(b"Press ENTER to reveal the secret...\n").unwrap();
    socket.read_exact(&mut [0u8]).unwrap();
    socket.write_all(format!("The secret is: {:?}\n", string).as_bytes()).unwrap();
}
