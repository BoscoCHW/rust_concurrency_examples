use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: i32,
}

impl Counter {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn inc(&mut self) {
        self.value += 1;
    }
}

fn inc(c: Arc<Mutex<Counter>>) {
    for _ in 0..500_000 {
        let mut counter = c.lock().unwrap();
        counter.inc();
    }
}

fn main() {
    let c = Arc::new(Mutex::new(Counter::new(0)));
    let mut handles = Vec::new();

    for _ in 0..3 {
        let c = Arc::clone(&c);
        let handle = thread::spawn(move || {
            inc(c);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counter = c.lock().unwrap();
    println!("{}", counter.value);
}

