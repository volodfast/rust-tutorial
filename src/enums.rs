pub fn main() {
  enum_example();

  enum_with_params();
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
