#[test]
fn test_tup_1() {
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    let (x, y, z) = tup;
    println!("x is {:?}", x);
}

#[test]
fn test_tup_2() {
    let s1 = String::from("hello");
    let (s1, size) = calculate_length(s1);
    println!("{:?} len is {:?}", s1, size);
}

fn calculate_length(s1: String) -> (String, u8) {
    let len = s1.len();
    (s1, len as u8)
}