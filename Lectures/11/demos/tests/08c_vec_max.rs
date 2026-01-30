//! Run this file with `cargo test --test 08c_vec_max`.

// TODO: Implement a function that finds the maximum number of a vector
// Implement it manually with a `for` cycle or `loop`.
//
// The input argument is a vector of `i32` integers.
// How does that type look like in Rust?

enum OptionalMax {
    Some(Vec<i32>),
    None,
}

impl OptionalMax {
    fn expect(self: OptionalMax, msg: &str) -> &Vec<i32> {
        match self {
            OptionalMax::Some(i) => i,
            OptionalMax::None => panic!("error: {msg}"),
        }
    }
}

// If v is empty, returns i32::MIN
fn get_max(v: &Vec<i32> ) -> Option<i32> {
    if v.len() == 0 {
        return Option::None;
    }
    let mut m = i32::MIN;
    for item in v {
        if *item > m {
            m = *item;
        }
    }
    Option::Some(m)
}

fn main() {
    let mut v = Vec::new();
    v.push(i32::MIN);
    v.push(i32::MIN);
    v.push(i32::MIN);
    let max = get_max(&v);

    match max {
        Some(max) => {
            
        },
        None => panic!("this shuld never happen"),
    }
}