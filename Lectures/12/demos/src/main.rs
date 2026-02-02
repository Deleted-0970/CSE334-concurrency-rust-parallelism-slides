use std::fmt::Display;

fn say_hello(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

struct Wow;

struct Hi {
    a: Wow,
    b: i64,
}

fn main() {
    let mut v = Vec::new();
    v.push(2);
    v.push(4);

    let r = &v[1];

    // v.push(4);

    println!("{r}");
    let mut asdf = (1, 2);
    asdf.0 += 1;
    // let mut H = Hi {a: Wow, b: 1};
    // let p = H.a;
    // let aa = &mut H.a;
    // let bb = &mut H.b;
    // *aa = 1.0;
    // *bb = 2;
    // let hh = &mut H;
    // println!("{aa} {hh}");
    // let v1 = &mut v[j];
    // let v2 = &mut v[i];
    // *v1 = 2;
    // *v2 = 3;


    

    // let r = say_hello(&v);
    // println!("Hello world! {r}");
}

impl Display for Hi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}