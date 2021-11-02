pub fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("{}, world! {}", s1, s2);

  let s = String::from("Hello"); // s comes into scope

  takes_ownership(s); // 's' value moves into the function and is no longer valid in this scope

  let x = 5; // x comes into scope

  makes_copy(x); // 'x' value moves into the function, but i32 has/is Copy, so it is still valid in this scope
} // x goes out of scope, s goes out of scope. Because 's' value was moved nothing special happens.

fn takes_ownership(some_string: String) {
  // some_string comes into scope
  println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
  // some_integer comes into scope
  println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.
