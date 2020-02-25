fn main() {
    let s = String::from("hello world");
    let res = first_word(&s);

    println!("result: {}", res);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);

    let word = first_word2(&s);
    println!("word: {}", word);

    let mut string = String::from("hello world");
    string.push_str("!!");
    let word = first_word2(&string);
    // string.push_str("!!");
    // error[E0502]: cannot borrow `string` as mutable because it is also borrowed as immutable
    //   --> src/slices.rs:19:3
    //   |
    // 18 |   let word = first_word2(&string);
    //   |                          ------- immutable borrow occurs here
    // 19 |   string.push_str("!!");
    //   |   ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // 20 |   println!("word: {}", word);
    //   |                        ---- immutable borrow later used here
    println!("word: {}", word);

    let my_string = String::from("Hello world");
    println!("word: {}", first_word3(&my_string[..]));

    let my_string_literal = "Hello world";
    println!("word: {}", first_word3(&my_string_literal[..]));

    println!("word: {}", first_word3(my_string_literal));

    let mut f = String::from("hello");
    f.push_str(", world");
    println!("message: {}", join(&f));
    f.push_str(", world");
    println!("message: {}", f);
    f.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn join(s: &String) -> String {
    // let new_str = format!("{}{}", s, "hoge");
    // return new_str;
    // format!("{}{}", s, "hoge")
    s.to_string() + "hoge"
    // s.clone() + "hoge"
}
