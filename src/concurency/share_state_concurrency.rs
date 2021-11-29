pub fn main() {
  // mutex_basics();
  multiple_thread_mutex_access();
}

#[allow(dead_code)]
fn mutex_basics() {
  use std::sync::Mutex;

  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}

#[allow(dead_code)]
fn multiple_thread_mutex_access() {
  use std::sync::{Arc, Mutex};
  use std::thread;

  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}
