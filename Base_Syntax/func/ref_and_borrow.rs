fn main() {
    let s1 = String::from("Hello");
    let len = calc_len(&s1);
    println!("len of '{}' is {}", s1, len);
    //len of 'Hello' is 5
}

fn calc_len(s: &String) -> usize {
    // 采用引用方式, 不用传入字符串, 所以也不会导致 string 所有权的转移
    s.len()
}
