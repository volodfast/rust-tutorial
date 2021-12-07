pub fn main() {
  type_synonyms();
}

fn type_synonyms() {
  type Kilometers = i32;

  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);
}
