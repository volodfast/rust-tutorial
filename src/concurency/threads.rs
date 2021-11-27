pub fn main() {
  thread_basics();
  move_values_in_thread_closure();
}

fn thread_basics() {
  use std::thread;
  use std::time::Duration;

  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}

fn move_values_in_thread_closure() {
  use std::thread;

  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();
}
