pub fn main() {
  basic();
  function_params();
  destructuring_struct();
  matching_enum();
  destructuring_nested_structs_and_enums();
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

fn destructuring_struct() {
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}

fn matching_enum() {
  enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::_Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::_Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y)
    }
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, blue {}", r, g, b)
    }
    Message::_Write(text) => println!("Text message: {}", text),
  }
}

fn destructuring_nested_structs_and_enums() {
  enum Color {
    _Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }

  enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(Color),
  }

  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message::ChangeColor(Color::_Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
      "Change the color to hue {}, saturation {}, and value {}",
      h, s, v
    ),
    _ => (),
  }
}
