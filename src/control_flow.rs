pub fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  let a = if true { 5 } else { 2 };

  println!("The value of a is: {}", a);

  loop_labels();
}

fn loop_labels() {
  let mut count = 0;

  'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;

    loop {
      println!("remaining = {}", remaining);
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {}", count);
}
