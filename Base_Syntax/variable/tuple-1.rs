fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (4, 3.2, 1);
    let (x, y, z) = tup;
    println!("x={},y={},z={}", x, y, z);
    println!("x={},y={},z={}", tup.0, tup.1, tup.2);
}
