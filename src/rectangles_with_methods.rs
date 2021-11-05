pub fn main() {
  method_in_rectangle();

  associated_functions();
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

    fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
    }
  }

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_functions() {
  #[derive(Debug)]
  struct Rectangle {
    height: u32,
    width: u32,
  }

  impl Rectangle {
    fn square(size: u32) -> Rectangle {
      Rectangle {
        width: size,
        height: size,
      }
    }
  }

  let rect = Rectangle::square(30);

  println!("Square: {:#?}", rect);
}
