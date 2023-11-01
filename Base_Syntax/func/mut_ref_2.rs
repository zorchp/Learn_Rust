fn t1() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}

fn t2() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
}

fn main() {
    // t1();
    t2();
}
