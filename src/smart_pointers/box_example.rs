pub fn main() {
  box_basics();
  box_for_recursive_types();
}

fn box_basics() {
  let b = Box::new(5);

  println!("b = {}", b);
}

fn box_for_recursive_types() {
  #[derive(Debug)]
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  println!("List: {:?}", list);
}
