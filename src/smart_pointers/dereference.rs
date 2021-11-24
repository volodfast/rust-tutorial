use std::ops::Deref;

pub fn main() {
  dereference_basics();
  using_box_like_reference();
  custom_made_smart_pointer();
}

fn dereference_basics() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

fn using_box_like_reference() {
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn custom_made_smart_pointer() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
