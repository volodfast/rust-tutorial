pub fn main() {
  basic();
  function_params();
}

fn basic() {
  let (_x, _y, _z) = (1, 2, 3);
}

fn function_params() {
  fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
  }

  let point = (3, 5);
  print_coordinates(&point);
}
