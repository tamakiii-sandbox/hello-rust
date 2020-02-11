
fn sub() {
  let x = 5;

  let x = x + 1;
  let x = x * 2;

  println!("The value of x is: {}", x);
}

fn spaces() {
  let spaces = "    ";
  let spaces = spaces.len();

  println!("Spaces: {}", spaces);
}

fn guess() {
  // let guess = "42".parse().expect("Not a number!");
  //   error[E0282]: type annotations needed
  //   --> src/variables.rs:19:7
  //   |
  // 19 |   let guess = "42".parse().expect("Not a number!");
  //   |       ^^^^^ consider giving `guess` a type
}

fn floating() {
  let x = 2.0;
  let y = 3.0;

  println!("x: {}", x);
  println!("y: {}", y);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("Max point: {}", MAX_POINTS);

    sub();
    spaces();
    guess();
    floating();
}
