pub fn main() {
  // thread_basics();
  // move_values_in_thread_closure();
  message_between_threads();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn move_values_in_thread_closure() {
  use std::thread;

  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();
}

#[allow(dead_code)]
fn message_between_threads() {
  use std::sync::mpsc;
  use std::thread;

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Got: {}", received);
}
