pub fn main() {
  enum_example();

  enum_with_params();

  match_example();

  match_with_option();

  if_let_example();
}

fn enum_example() {
  #[derive(Debug)]
  enum IpAddrKind {
    V4,
    V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  println!("IPV4: {:#?}, IPV6: {:#?}", four, six);
}

fn enum_with_params() {
  #[derive(Debug)]
  enum IpAddr {
    V4(String),
    V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));

  println!("Home: {:#?}, Loopback: {:#?}", home, loopback);
}

fn match_example() {
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }

  println!("Penny: {}", value_in_cents(Coin::Penny));
  println!("Nickel: {}", value_in_cents(Coin::Nickel));
  println!("Dime: {}", value_in_cents(Coin::Dime));
  println!("Quarter: {}", value_in_cents(Coin::Quarter));
}

fn match_with_option() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("Five: {:?}, Six: {:?}, None: {:?}", five, six, none);
}

fn if_let_example() {
  let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }
}
