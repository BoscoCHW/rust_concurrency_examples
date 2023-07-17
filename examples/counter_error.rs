use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: i32,
    mutex: Mutex<()>,
}

impl Counter {
    fn new(value: i32) -> Self {
        Self {
            value,
            mutex: Mutex::new(()),
        }
    }

    fn inc(&mut self) {
        let _guard = self.mutex.lock().unwrap();
        self.value += 1;
    }
}

fn inc(c: Arc<Counter>) {
    for _ in 0..500_000 {
        c.inc();   // cannot borrow data in an `Arc` as mutable
    }
}

fn main() {
    let c = Arc::new(Counter::new(0));
    let mut handles = Vec::new();

    for _ in 0..2 {
        let c = Arc::clone(&c);
        let handle = thread::spawn(move || {
            inc(c);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", c.value);
}
