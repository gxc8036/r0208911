#[test]
fn test_tup_1() {
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    let (x, y, z) = tup;
    println!("x is {:?}", x);
}