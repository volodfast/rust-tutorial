pub fn main() {
  vector_introduction();

  reading_vector();

  iterating_over_vector();

  using_enums_with_vectors();
}

fn vector_introduction() {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  println!("Vector: {:?}", v);
}

fn reading_vector() {
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }
}

fn iterating_over_vector() {
  let mut v = vec![100, 32, 57];

  for i in &v {
    println!("{}", i);
  }

  for i in &mut v {
    *i += 50;
  }

  println!("Vector: {:?}", v);
}

fn using_enums_with_vectors() {
  #[derive(Debug)]
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  println!("The spreadsheet row is: {:#?}", row);
}
