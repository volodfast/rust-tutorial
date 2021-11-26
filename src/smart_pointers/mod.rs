mod box_example;
mod dereference;
mod drop_example;
mod interior_mutability;
mod rc_example;

pub fn main() {
  box_example::main();
  dereference::main();
  drop_example::main();
  rc_example::main();
  interior_mutability::main();
}
