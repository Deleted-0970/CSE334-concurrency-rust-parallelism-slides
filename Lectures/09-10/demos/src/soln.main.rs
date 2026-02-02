//fn hello(a: Vec<i32>) -> i32 {
fn hello(a: &Vec<i32>) -> i32 {
    println!("ok");

    a.into_iter().sum()

    /*

    let mut s = 0;
    for n in a {
        s += n;
    }
    s
    */
    
    /*
    let mut ind = 0;
    let mut s = 0;
    while ind < a.len() {
        s += a[ind];
        ind+=1;
    }
    s
    */
}

fn main() {
    let mut v = Vec::new();
    v.push(3);
    v.push(4);
    //let v = v;
    //let foo = v;
    //let bar = &v;
    //let foobar = &mut v;
    //let s: i32 = bar.iter().sum();
    //let n = hello(v);
    //let n2 = hello(v);
    let n = hello(&v);
    let n2 = hello(&v);
    println!("Hello, world! n={n}");
}
