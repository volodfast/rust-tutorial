use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn main() {
  // panic_macro();
  // open_file();
  // propagating_errors();
  // propagating_errors_with_question_mark();
  propagating_errors_chaining();
}

fn panic_macro() {
  // panic!("crash and burn");
  let v = vec![1, 2, 3];

  v[99];
}

fn open_file() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error);
      }
    },
  };

  println!("{:?}", f);
}

fn propagating_errors() {
  fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
    }
  }

  let f = read_username_from_file();

  println!("{:?}", f);
}

fn propagating_errors_with_question_mark() {
  fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
  }

  let f = read_username_from_file();

  println!("{:?}", f);
}

fn propagating_errors_chaining() {
  fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
  }

  let f = read_username_from_file();

  println!("{:?}", f);
}
