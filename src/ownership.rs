fn main() {
  // let s = "hello";
  // println!("message: {}", s);
  // #[prelude_import]
  // use std::prelude::v1::*;
  // #[macro_use]
  // extern crate std;
  // fn main() {
  //     let s = "hello";
  //     {
  //         ::std::io::_print(::core::fmt::Arguments::new_v1(
  //             &["message: ", "\n"],
  //             &match (&s,) {
  //                 (arg0,) => [::core::fmt::ArgumentV1::new(
  //                     arg0,
  //                     ::core::fmt::Display::fmt,
  //                 )],
  //             },
  //         ));
  //     };
  // }

  // let mut s = "hello";
  // s.push_str(", world!");
  // println!("message: {}", s);
  // error[E0599]: no method named `push_str` found for type `&str` in the current scope
  //   --> src/ownership.rs:28:5
  //   |
  // 28 |   s.push_str(", world!");
  //   |     ^^^^^^^^ method not found in `&str`

  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("message: {}", s);

  let s1 = String::from("hello");
  let s2 = s1;

  // println!("message: {}", s1);
  // error[E0382]: borrow of moved value: `s1`
  //   --> src/ownership.rs:39:27
  //   |
  // 36 |   let s1 = String::from("hello");
  //   |       -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
  // 37 |   let s2 = s1;
  //   |            -- value moved here
  // 38 | 
  // 39 |   println!("message: {}", s1);
  //   |                           ^^ value borrowed here after move
  println!("message: {}", s2);

  let s1 = String::from("hello");
  let mut s2 = s1.clone();
  s2.push_str(", world");

  println!("message: {} and {}", s1, s2);

  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);

  let s = String::from("hello");
  takes_ownership(s);
  // println!("message: {}", s);
  // error[E0382]: borrow of moved value: `s`
  //   --> src/ownership.rs:65:27
  //   |
  // 63 |   let s = String::from("hello");
  //   |       - move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
  // 64 |   takes_ownership(s);
  //   |                   - value moved here
  // 65 |   println!("message: {}", s);
  //   |                           ^ value borrowed here after move

  let s1 = String::from("Hello");
  let (s2, len) = calculate_length(s1);
  println!("The length of {} is {}", s2, len);
}

fn takes_ownership(message: String) {
  println!("message: {}", message);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}