use std::collections::HashMap;

pub fn main() {
  creating();

  accessing_values();
}

fn creating() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  println!("{:?}", scores);

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

  println!("Scores: {:?}", scores);
}

fn accessing_values() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  println!("{:?}", score);

  for (key, value) in scores {
    println!("{}: {}", key, value);
  }
}
