pub fn main() {
  basic_rc_example();
  reference_count();
}

#[derive(Debug)]
enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn basic_rc_example() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  let b = Cons(3, Rc::clone(&a));
  let c = Cons(4, Rc::clone(&a));

  println!("b = {:?}", b);
  println!("c = {:?}", c);
}

fn reference_count() {
  println!("||||");
  println!("reference_count");
  println!("||||");
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("b = {:?}", b);
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("c = {:?}", c);
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
