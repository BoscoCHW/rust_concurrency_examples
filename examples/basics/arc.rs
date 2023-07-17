use std::sync::Arc;

fn main() {
  let shared_value = 5;
  let shared_value_ptr1 = Arc::new(shared_value);
  let shared_value_ptr2 = shared_value_ptr1.clone();
  
  let handle1 = std::thread::spawn(move || {
      println!("Value: {}", *shared_value_ptr1);
  });

  let handle2 = std::thread::spawn(move || {
    println!("Value: {}", *shared_value_ptr2);
});
  
  // Wait for the thread to finish
  handle1.join().unwrap();
  handle2.join().unwrap();
}
