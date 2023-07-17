use std::sync::{Arc, Condvar, Mutex};
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let mut handles = vec![];
    for i in 0..5 {
        let pair_clone = pair.clone();
        let handle = std::thread::spawn(move || {
            let (lock, cvar) = &*pair_clone;
            let mut start = lock.lock().unwrap();
            while !*start {                // barrier to start
                start = cvar.wait(start).unwrap();
            }
            println!("started thread {}", i);
        });
        handles.push(handle);
    }
    // Notify the thread that the condition has been met
    {
        let (lock, cvar) = &*pair;
        let mut done = lock.lock().unwrap();
        *done = true;
        cvar.notify_all();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
