fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    change(&mut s1);
    println!("changed message: {}", s1);

    change(&mut s1);
    println!("changed message: {}", s1);

    let r1 = &mut s1;
    change(r1);
    println!("changed message: {}", s1);

    let r2 = &mut s1;
    change(r2);
    println!("changed message: {}", s1);

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // change(r1);
    // change(r2);
    // println!("message: {}", s);
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //   --> src/ref-and-borrow.rs:24:12
    //   |
    // 23 |   let r1 = &mut s;
    //   |            ------ first mutable borrow occurs here
    // 24 |   let r2 = &mut s;
    //   |            ^^^^^^ second mutable borrow occurs here
    // 25 |
    // 26 |   change(r1);
    //  |          -- first borrow later used here

    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(message: &String) {
//   message.push_str(", world");
// }
// error[E0596]: cannot borrow `*message` as mutable, as it is behind a `&` reference
//   --> src/ref-and-borrow.rs:16:3
//    |
// 15 | fn change(message: &String) {
//    |                    ------- help: consider changing this to be a mutable reference: `&mut std::string::String`
// 16 |   message.push_str(", world");
//    |   ^^^^^^^ `message` is a `&` reference, so the data it refers to cannot be borrowed as mutable

fn change(message: &mut String) {
    message.push_str(", world");
}

// fn dangle() -> &String {
//   let s = String::from("hello");
//   &s
// }
// error[E0106]: missing lifetime specifier
//   --> src/ref-and-borrow.rs:61:16
//    |
// 61 | fn dangle() -> &String {
//    |                ^ help: consider giving it a 'static lifetime: `&'static`
//    |
//    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
