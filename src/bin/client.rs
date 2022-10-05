use std::io;
use std::io::Read;
use std::str;
use std::net::TcpStream;

fn main() {
//    let mut sstream = TcpStream::connect("127.0.0.1:9090").unwrap();
    let mut stdin = io::stdin();
    let mut buffer : [u8; 50] = [0; 50];

    //stream.set_nonblocking(true).expect("Cannot set non-blocking");
    loop {
        match stdin.read(&mut buffer) {
            Ok(n) => println!(
                    "Read {} bytes: {}",
                    n,
                    str::from_utf8(&buffer).unwrap()
                ),
            Err(_) => panic!("encounterd IO error"),
        };
    }
}
