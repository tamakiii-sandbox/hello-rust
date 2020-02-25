macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = Vec::new();

            $(
                tmp.push($x);
            )*

            tmp
        }
    };
}

fn main() {
    let x: Vec<u32> = myvec!(1, 2, 3);

    println!("hello: {:?}", x);
}
