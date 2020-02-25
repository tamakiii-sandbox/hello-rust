macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = Vec::new();

            $(
                tmp.push($x);
            )*

            tmp
        }
    }
}

macro_rules! foo {
    (x => $e:expr) => {
        println!("mode X: {}", $e);
    };
    (y => $e:expr) => {
        println!("mode Y: {}", $e);
    };
}

fn main() {
    let x: Vec<u32> = myvec!(1, 2, 3);
    println!("hello: {:?}", x);

    foo!(y => 2);
}
