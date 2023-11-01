fn main() {
    // let s = String::from("hello");
    // 引用默认不可变, 不可修改引用指向的值
    // change(&s); //error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    let mut s1 = String::from("hello");
    change1(&mut s1);
    println!("{}", s1);
}

// fn change(s: &String) {
//     s.push_str(", world");
// }

fn change1(s: &mut String) {
    s.push_str(", world");
}
