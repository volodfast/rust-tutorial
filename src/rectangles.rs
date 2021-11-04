pub fn main() {
  without_struct();

  refactoring_with_tuples();

  refactoring_with_structs();

  derived_traits();
}

fn without_struct() {
  fn area(width: u32, height: u32) -> u32 {
    width * height
  }
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
}

fn refactoring_with_tuples() {
  fn area(dimesions: (u32, u32)) -> u32 {
    dimesions.0 * dimesions.1
  }

  let rect1 = (30, 50);

  println!("The area of the rectangle is {} square pixels", area(rect1));
}

fn refactoring_with_structs() {
  struct Rectangle {
    width: u32,
    height: u32,
  }

  fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
  }

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels",
    area(&rect1)
  );
}

fn derived_traits() {
  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }

  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  println!("rect1 is {:#?}", rect1);

  dbg!(&rect1);
}
