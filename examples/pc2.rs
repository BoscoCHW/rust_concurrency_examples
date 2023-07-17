use rand::Rng;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Buffer {
    cap: i32,
    len: Mutex<i32>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl Buffer {
    fn new(cap: i32) -> Self {
        Buffer {
            cap,
            len: Mutex::new(0),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    fn push(&self) {
        let mut len = self.len.lock().unwrap();
        while *len == self.cap {
            len = self.not_full.wait(len).unwrap();
        }
        *len += 1;
        self.not_empty.notify_one();
        print!("+");
        std::io::stdout().flush().unwrap();
    }

    fn pop(&self) {
        let mut len = self.len.lock().unwrap();
        while *len == 0 {
            len = self.not_empty.wait(len).unwrap();
        }
        *len -= 1;
        self.not_full.notify_one();
        print!("-");
        std::io::stdout().flush().unwrap();
    }
}

fn produce(buffer: Arc<Buffer>) {
    loop {
        buffer.push();
        work(50, 100);
    }
}

fn consume(buffer: Arc<Buffer>) {
    loop {
        buffer.pop();
        work(50, 100);
    }
}

fn work(min: u64, max: u64) {
    let duration = Duration::from_millis(rand::thread_rng().gen_range(min..max));
    thread::sleep(duration);
}

fn main() {
    println!("starting...");
    let buffer = Arc::new(Buffer::new(100));

    let mut handles = vec![];

    for _ in 0..5 {
        let buffer = buffer.clone();
        handles.push(thread::spawn(move || {
            produce(buffer);
        }));
    }

    for _ in 0..3 {
        let buffer = buffer.clone();
        handles.push(thread::spawn(move || {
            consume(buffer);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
