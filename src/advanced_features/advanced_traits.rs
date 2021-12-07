pub fn main() {
  operator_overloading();
  advanced_add_output();
  using_supertraits();
}

fn operator_overloading() {
  use std::ops::Add;

  #[derive(Debug, Copy, Clone, PartialEq)]
  struct Point {
    x: i32,
    y: i32,
  }

  impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }

  assert_eq!(
    Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    Point { x: 3, y: 3 },
  )
}

fn advanced_add_output() {
  use std::ops::Add;

  #[derive(Debug, PartialEq)]
  struct Millimeters(u32);
  struct Meters(u32);

  impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }

  let mili = Millimeters(10);
  let met = Meters(1);

  assert_eq!(mili + met, Millimeters(1010));
}

fn using_supertraits() {
  use std::fmt;

  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();

      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }

  struct Point {
    x: i32,
    y: i32,
  }

  impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "x: {}, y: {}", self.x, self.y)
    }
  }

  impl OutlinePrint for Point {}

  let point = Point { x: 1, y: 2 };

  point.outline_print();
}
