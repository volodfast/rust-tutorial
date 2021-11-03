pub fn main() {
  struct_instantiation();

  struct_builder();

  tuple_struct_example();
}

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn struct_instantiation() {
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  display_user(&user1);

  let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user2.email = String::from("anotheremail@example.com");

  display_user(&user2);
}

fn display_user(user: &User) {
  println!(
    "{} {} {} {}",
    user.email, user.username, user.active, user.sign_in_count
  );
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

fn struct_builder() {
  let user = build_user(String::from("somerandom@email.com"), String::from("randy"));

  display_user(&user);
}

fn tuple_struct_example() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("{} {}", black.0, origin.0)
}
