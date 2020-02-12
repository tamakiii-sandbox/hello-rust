
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  // if number {
  //   println!("number was ...?");
  // }
  //   error[E0308]: mismatched types
  //   --> src/branches.rs:11:6
  //   |
  // 11 |   if number {
  //   |      ^^^^^^ expected `bool`, found integer

  let number = if true {
    5
  } else {
    6
  };

  println!("The value of number is: {}", number);

  // let number = if true {
  //   5
  // } else {
  //   "six"
  // };

  // println!("The value of number is: {}", number);
  //   error[E0308]: if and else have incompatible types
  //   --> src/branches.rs:31:5
  //   |
  // 28 |     let number = if true {
  //   |  ________________-
  // 29 | |     5
  //   | |     - expected because of this
  // 30 | |   } else {
  // 31 | |     "six"
  //   | |     ^^^^^ expected integer, found `&str`
  // 32 | |   };
  //   | |___- if and else have incompatible type


  let a = [10, 20, 30, 40, 50,];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
}