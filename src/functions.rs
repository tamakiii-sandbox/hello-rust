
fn main() {
  another_functions(5);
  let_block();
  println!("Result of five() is: {}", five());
}

fn another_functions(x: i32) {
  println!("the value of x is: {}", x);
}

// fn another_functions(x) {
//   println!("the value of x is: {}", x);
// }
// error: expected one of `:`, `@`, or `|`, found `)`
//  --> src/functions.rs:7:23
//   |
// 7 | fn another_functions(x) {
//   |                       ^ expected one of `:`, `@`, or `|`
//   |
//   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
// help: if this was a parameter name, give it a type
//   |
// 7 | fn another_functions(x: TypeName) {
//   |                      ^^^^^^^^^^^
// help: if this is a type, explicitly ignore the parameter name
//   |
// 7 | fn another_functions(_: x) {

fn let_block() {
  let x = 5;
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {}", y);
  println!("The value of x is: {}", x);
}

fn five() -> i32 {
  45 + 1
  // 45 + 1;
  // error[E0308]: mismatched types
  //   --> src/functions.rs:40:14
  //   |
  // 40 | fn five() -> i32 {
  //   |    ----      ^^^ expected `i32`, found `()`
  //   |    |
  //   |    implicitly returns `()` as its body has no tail or `return` expression
  // 41 |   45 + 1;
  //   |         - help: consider removing this semicolon
}