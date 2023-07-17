use std::sync::{Arc, Mutex};
use futures::future::join_all;

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

async fn inc(c: Arc<Mutex<Counter>>) {
    for _ in 0..500_000 {
        let mut counter = c.lock().unwrap();
        counter.inc();
    }
}

// #[tokio::main]
fn main() {
    let c = Arc::new(Mutex::new(Counter::new(0)));

    let tasks = vec![inc(Arc::clone(&c)), inc(Arc::clone(&c)), inc(Arc::clone(&c))];

    futures::executor::block_on(join_all(tasks));
    // join_all(tasks).await;

    let counter = c.lock().unwrap();
    println!("{}", counter.value);
}
