use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: Mutex<u32>,
}

impl Counter {
    fn new() -> Self {
        Counter { count: Mutex::new(0) }
    }

    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    fn get_count(&self) -> u32 {
        let count = self.count.lock().unwrap();
        *count
    }
}

fn inc(c: Arc<Counter>) {
  for _ in 0..500_000 {
      c.increment();
  }
}

fn main() {
    let counter = Arc::new(Counter::new());

    let mut handles = vec![];
    for _ in 0..3 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            inc(counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get_count());
}
