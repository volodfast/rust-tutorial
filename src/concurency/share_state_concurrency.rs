pub fn main() {
  mutex_basics();
}

fn mutex_basics() {
  use std::sync::Mutex;

  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}
