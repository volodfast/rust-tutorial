pub fn main() {
  enum_example();
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