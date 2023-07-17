use rand::Rng;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Buffer {
    cap: i32,
    len: i32,
}

impl Buffer {
    fn new(cap: i32) -> Self {
        Buffer { cap, len: 0 }
    }

    fn push(&mut self) {
        self.len += 1;
        print!("+");
        std::io::stdout().flush().unwrap();
    }

    fn pop(&mut self) {
        self.len -= 1;
        print!("-");
        std::io::stdout().flush().unwrap();
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn produce(buffer: Arc<Mutex<Buffer>>, conds: Arc<(Condvar, Condvar)>) {
    let (not_empty, not_full) = &*conds;

    loop {
        let mut buffer = buffer.lock().unwrap();
        while buffer.is_full() {
            buffer = not_full.wait(buffer).unwrap();
        }
        buffer.push();
        work(50, 100);
        not_empty.notify_one();
    }
}

fn consume(buffer: Arc<Mutex<Buffer>>, conds: Arc<(Condvar, Condvar)>) {
    let (not_empty, not_full) = &*conds;
    loop {
        let mut buffer = buffer.lock().unwrap();
        while buffer.is_empty() {
            buffer = not_empty.wait(buffer).unwrap();
        }
        buffer.pop();
        work(50, 100);
        not_full.notify_one();
    }
}

fn work(min: u64, max: u64) {
    let duration = Duration::from_millis(rand::thread_rng().gen_range(min..max));
    thread::sleep(duration);
}

fn main() {
    println!("starting...");
    let buffer = Arc::new(Mutex::new(Buffer::new(100)));
    let conds = Arc::new((Condvar::new(), Condvar::new()));
    let mut handles = vec![];

    for _ in 0..3 {
        let buffer = buffer.clone();
        let conds = conds.clone();
        handles.push(thread::spawn(move || {
            produce(buffer, conds);
        }));
    }

    for _ in 0..3 {
        let buffer = buffer.clone();
        let conds = conds.clone();
        handles.push(thread::spawn(move || {
            consume(buffer, conds);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
