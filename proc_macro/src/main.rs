use proc_macro::trace;

#[trace]
fn test() {
    println!("Hello, world!");
}

fn main() {
    test();
}
