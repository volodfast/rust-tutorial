pub fn main() {
  method_in_rectangle();
}

fn method_in_rectangle() {
  #[derive(Debug)]
  struct Rectangle {
    height: u32,
    width: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );
}
