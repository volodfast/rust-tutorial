pub fn numeric_operations() {
  // addition
  let sum = 5 + 10;

  // substraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let floored = 2 / 3; // Results in 0

  // remainder
  let remainder = 43 % 5;
}

pub fn compound_types() {
  // tuple
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of y is: {}", y);

  let five_hundred = tup.0;

  let six_point_four = tup.1;

  let one = tup.0;

  let a = [1, 2, 3, 4, 5];

  let first = a[0];

  let second = a[1];
}
