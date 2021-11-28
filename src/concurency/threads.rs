pub fn main() {
  // thread_basics();
  // move_values_in_thread_closure();
  // message_between_threads();
  multiple_message();
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

#[allow(dead_code)]
fn multiple_message() {
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for recived in rx {
    println!("Got: {}", recived);
  }
}
