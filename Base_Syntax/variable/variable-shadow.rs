fn main() {
    // t1();
    t2();
}
fn t1() {
    let x = 5;
    println!("{}", x);
    let x = x + 1;
    println!("{}", x);
    let x = x * 2;
    println!("{}", x);
}

fn t2() {
    let sps = "   ";
    let sps = sps.len();
    println!("sps.len = {}", sps); //3
}
fn t3() {
    let mut sps = "   ";
    sps = sps.len();
    println!("sps.len = {}", sps); //error[E0308]: mismatched types
}
