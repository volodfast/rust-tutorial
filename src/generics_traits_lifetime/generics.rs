pub fn main() {
  // find_largest_example();

  method_definition_generic();
}

// fn find_largest_example() {
//   fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//       if item > largest {
//         largest = item;
//       }
//     }

//     largest
//   }

//   let number_list = vec![34, 50, 25, 100, 65];

//   let result = largest(&number_list);
//   println!("The largest number is {}", result);

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest(&char_list);
//   println!("The largest char is {}", result);
// }

fn method_definition_generic() {
  struct Point<T> {
    x: T,
    y: T,
  }

  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }

  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }

  let p = Point { x: 5, y: 10 };

  println!("p.x = {}", p.x);

  let p = Point {
    x: 10.0f32,
    y: 20.0f32,
  };

  println!("p.distance_from_origin = {}", p.distance_from_origin());
}
