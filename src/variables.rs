fn lets() {
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

fn boolean() {
    let t = true;
    let f: bool = false;

    println!("boolean: {}", t);
    println!("boolean: {}", f);
}

fn chars() {
    let z = 'ℤ';
    let cat = '😻';

    println!("z: {}", z);
    println!("cat: {}", cat);
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, y, _) = tup;
    // let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);

    // println!("tup: {}", tup);
    //   error[E0277]: `(i32, f64, u8)` doesn't implement `std::fmt::Display`
    //   --> src/variables.rs:54:23
    //   |
    // 54 |   println!("tup: {}", tup);
    //   |                       ^^^ `(i32, f64, u8)` cannot be formatted with the default formatter
    //   |
    //   = help: the trait `std::fmt::Display` is not implemented for `(i32, f64, u8)`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: required by `std::fmt::Display::fmt`
}

fn arrays() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let last = months[months.len() - 1];

    println!("first: {}", first);
    println!("last: {}", last);

    // let element = months[100];
    // println!("element is: {}", element);
    // cargo run --bin variables
    //   Compiling hello v0.1.0 (/workspaces/hello-rust)
    // error: index out of bounds: the len is 12 but the index is 100
    //   --> src/variables.rs:93:19
    //   |
    // 93 |     let element = months[100];
    //   |                   ^^^^^^^^^^^
    //   |
    //   = note: `#[deny(const_err)]` on by default
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("Max point: {}", MAX_POINTS);

    lets();
    spaces();
    guess();
    floating();
    boolean();
    chars();
    tuples();
    arrays();
}
