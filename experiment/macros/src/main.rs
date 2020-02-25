
fn main() {
    let x: Vec<u32> = {
        let mut tmp = Vec::new();
        tmp.push(1);
        tmp.push(2);
        tmp.push(3);
        tmp
    };

    println!("hello: {:?}", x);
}
