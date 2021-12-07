pub fn main() {
  function_pointers();
}

fn function_pointers() {
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
  }

  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);
}
