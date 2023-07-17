use rand::Rng;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_condvar::Condvar;

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

async fn produce(buffer: Arc<Mutex<Buffer>>, conds: Arc<(Condvar, Condvar)>) {
    let (not_empty, not_full) = &*conds;
    loop {
        let mut buffer = buffer.lock().await;
        while buffer.is_full() {
            buffer = not_full.wait(buffer).await;
        }
        buffer.push();
        work(50, 100).await;
        not_empty.notify_one();
    }
}

async fn consume(buffer: Arc<Mutex<Buffer>>, conds: Arc<(Condvar, Condvar)>) {
    let (not_empty, not_full) = &*conds;
    loop {
        let mut buffer = buffer.lock().await;
        while buffer.is_empty() {
            buffer = not_empty.wait(buffer).await;
        }
        buffer.pop();
        work(50, 100).await;
        not_full.notify_one();
    }
}

async fn work(min: u64, max: u64) {
    let duration = Duration::from_millis(rand::thread_rng().gen_range(min..max));
    tokio::time::sleep(duration).await;
}

#[tokio::main]
async fn main() {
    println!("starting...");
    let buffer = Arc::new(Mutex::new(Buffer::new(100)));
    let conds = Arc::new((Condvar::new(), Condvar::new()));

    let mut handles = vec![];
    for _ in 0..4 {
        let buffer = buffer.clone();
        let conds = conds.clone();
        handles.push(tokio::spawn(async move {
            produce(buffer, conds).await;
        }));
    }

    for _ in 0..3 {
        let buffer = buffer.clone();
        let conds = conds.clone();
        handles.push(tokio::spawn(async move {
            consume(buffer, conds).await;
        }));
    }
    
    futures::future::join_all(handles).await;
}
