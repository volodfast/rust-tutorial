pub fn main() {
  creating_string();
  indexing();
}

fn creating_string() {
  let data = "initial contents";

  let mut s = data.to_string();

  s.push_str(" bar");

  s.push('!');

  println!("{}", s);

  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");

  let ttt = format!("{}-{}-{}", tic, tac, toe);

  println!("{}", ttt);
}

fn indexing() {
  let namaste = "नमस्ते";

  for c in namaste.chars() {
    println!("{}", c);
  }

  for b in namaste.bytes() {
    println!("{}", b);
  }
}
