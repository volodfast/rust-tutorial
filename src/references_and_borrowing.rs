// The rules of references
// 1. At any given time, you can have one mutable reference OR any number of immutable references (to specific value).
// 2. References must always be valid.

pub fn main() {
  references_as_function_arguments();

  mutable_references();
}

fn references_as_function_arguments() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
  // s is a reference to a String
  s.len()
} // Here, s goes out of scope, but because it does not have ownership of what it refers to, nothing happens.

fn mutable_references() {
  let mut s = String::from("Hello");

  change(&mut s);

  println!("Mutable reference s is: {}", s);
}

fn change(some_string: &mut String) {
  some_string.push_str(" World!");
}
