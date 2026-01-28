fn say_hello(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let mut v = Vec::new();
    v.push(2);
    v.push(4);

    let r = &v[1];

    // v.push(4);

    println!("{r}");

    

    // let r = say_hello(&v);
    // println!("Hello world! {r}");
}
