pub fn main() {
  basic_usage();
}

fn basic_usage() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  for val in v1_iter {
    println!("Got: {}", val);
  }
}
