use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 1024];
    let mut count = 0;

    while handle.read(&mut buf).unwrap() > 0 {
        for byte in &buf {
            match byte {
                b'+' => {
                    println!("count: {}", count);
                    count += 1
                }
                b'-' => {
                    println!("count: {}", count);
                    count -= 1
                }
                _ => (),
            }
            if count < 0 || count > 100 {
                println!("ERROR!");
                return;
            }
        }
        buf = [0u8; 1024];
    }
}
