//! Run this file with `cargo test --test 08c_vec_max`.

// TODO: Implement a function that finds the maximum number of a vector
// Implement it manually with a `for` cycle or `loop`.
//
// The input argument is a vector of `i32` integers.
// How does that type look like in Rust?

enum OptionalMax {
    Something(i32),
    Nothing,
}

/*
enum ErrorType {
    EmptyList,
    MaxValueNegative,
}

enum ResultMax {
    AllGood(i32),
    Error(ErrorType),
}
*/

fn get_max(v: &Vec<i32>) -> OptionalMax {
    if v.len() == 0 {
        return OptionalMax::Nothing;
    }
    let mut cur_max = i32::MIN;
    for i in v {
        if cur_max < *i {
            cur_max = *i;
        }
    }
    OptionalMax::Something(cur_max)
}

impl OptionalMax {
    fn expect(self, err_msg: &str) {
        match self {
            OptionalMax::Nothing => {
                panic!(err_msg)
            }
            OptionalMax::Something(i) => return i,
        }
    }
    fn unwrap(self) {
        self.expect("unwrap failed!");
    }
}

fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let m = get_max(&v);
    let n = m.expect("this should never happen");
    // do stuff with n
}

/*
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let m = get_max(&v);
    let n = match m {
        OptionalMax::Nothing => panic!("this should never happen");
        OptionalMax::Something(i) => n
    };
    // do stuff with n
}
*/

/*
fn main() {
    let mut v = Vec::new();
    let m1 = get_max(&v); // OptionalMax::Nothing
    v.push(i32::MIN);
    let m2 = get_max(&v); // OptionalMax::Something(i32::MIN)
    match m2 {
        OptionalMax::Nothing => ...
        OptionalMax::Something(i) => {

        }
    }
}
*/

/*
fn get_max(v: &Vec<i32>) -> i32 {
    let mut cur_max = i32::MIN;
    for i in v {
        if cur_max < *i {
            cur_max = *i;
        }
    }
    cur_max
}

fn main() {
    let mut v = Vec::new();
    let m1 = get_max(&v);
    v.push(i32::MIN);
    let m2 = get_max(&v);
}
*/
