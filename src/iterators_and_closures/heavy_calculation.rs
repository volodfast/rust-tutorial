use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, A, R>
where
  T: Fn(A) -> R,
{
  calculation: T,
  hash: HashMap<A, R>,
}

impl<T, A, R> Cacher<T, A, R>
where
  T: Fn(A) -> R,
  A: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
  R: std::clone::Clone,
{
  fn new(calculation: T) -> Self {
    Self {
      calculation,
      hash: HashMap::new(),
    }
  }

  fn value(&mut self, arg: A) -> R {
    match self.hash.get(&arg) {
      Some(v) => v.clone(),
      None => {
        let result = (self.calculation)(arg.clone());
        let v = result.clone();
        self.hash.insert(arg.clone(), result);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_result.value(intensity));

    println!("Next, do {} situps", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cacher_stores_different_key_values() {
    let mut cache1 = Cacher::new(|n| n + 1);

    let mut cache2 = Cacher::new(|str: &str| format!("{} world", str.clone()));

    assert_eq!(cache1.value(1), 2);
    assert_eq!(cache2.value("hello"), "hello world");
  }
}
