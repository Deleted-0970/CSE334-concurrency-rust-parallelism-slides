//! Run this file with `cargo test --test 09a_iterator_demo`.

struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    // let list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let l = list.iter().filter(|x| {x.width == 3});

    let mut c = Counter {value: 4};

    loop {
        match c.next() {
            Some(d) => {

            },
            None => {break},
        }
    }
}

struct Counter {
    value: u64
}

impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}