use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread 1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread 2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
