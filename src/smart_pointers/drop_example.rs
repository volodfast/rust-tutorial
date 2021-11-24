pub fn main() {
  cleanup_cycle();
  dropping_value_early();
}

#[derive(Debug)]
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn cleanup_cycle() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };

  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };

  println!("{:?} and {:?} created.", c, d);
}

fn dropping_value_early() {
  println!("||||");
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };

  println!("{:?} created", c);
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}
