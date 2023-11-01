fn main() {
    // let ref_to_nothing = dangle(); //error[E0106]: missing lifetime specifier
    let okk = work();
    println!("{}", okk);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn work() -> String {
    let s = String::from("world");
    s
}
