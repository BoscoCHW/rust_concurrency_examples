use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct TwoStageBarrier {
    count: Mutex<usize>,
    cvar: Condvar,
    num_threads: usize,
}

impl TwoStageBarrier {
    fn new(n: usize) -> TwoStageBarrier {
        TwoStageBarrier {
            count: Mutex::new(0),
            cvar: Condvar::new(),
            num_threads: n,
        }
    }

    fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        if *count == self.num_threads {
            self.cvar.notify_all();
        } else {
            while *count < self.num_threads {
                count = self.cvar.wait(count).unwrap();
            }
        }

    }
}

fn main() {
    let num_threads = 3;
    let barrier = Arc::new(TwoStageBarrier::new(num_threads));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let b = barrier.clone();
        let handle = thread::spawn(move || {
            println!(
                "Thread {:?} is waiting at the barrier",
                thread::current().id()
            );
            b.wait();
            println!("Thread {:?} passed the barrier", thread::current().id());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


