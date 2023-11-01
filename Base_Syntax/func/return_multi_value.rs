fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calc_len(s1);
    println!("len of \"{}\" is {}", s2, len);
    //len of "hello" is 5
}

fn calc_len(s: String) -> (String, usize) {
    let _len = s.len();
    (s, _len)
}
