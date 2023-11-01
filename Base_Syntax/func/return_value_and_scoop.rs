fn main() {
    //将一个值赋值给另一个变量时就会转移所有权
    // 当一个持有堆数据的变量离开作用域时，它的数据就会被drop清理回收，除非这些数据的所有权移动到了另一个变量上。
    let _s1 = gives_ownership();
    let _s2 = String::from("hello");
    let _s3 = takes_and_gives_back(_s2);
}

fn gives_ownership() -> String {
    let st1 = String::from("hello");
    st1
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
