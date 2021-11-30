pub fn main() {
  let mut col = AveragedCollection {
    list: vec![],
    average: 0.0,
  };

  col.add(3);
  col.add(5);
  col.remove();

  println!("Average: {}", col.average());
}

pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();

    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();

    self.average = total as f64 / self.list.len() as f64;
  }
}
