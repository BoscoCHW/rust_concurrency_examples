use std::sync::{Arc, Mutex};

fn main() {
    let mutex = Mutex::new(0);
    let mutex_ptr = Arc::new(mutex);
    let handles = vec![];

    for _ in 0..2 {
        let mutex_ptr = mutex_ptr.clone();
        let handle = std::thread::spawn(move || {
            let mut value = mutex_ptr.lock().unwrap();
            *value += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // Get the value of the mutex
    let value = mutex_ptr.lock().unwrap();
    println!("Value: {}", *value);
}
