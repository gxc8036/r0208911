#[derive(Debug)]
struct User {
    activate: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

#[test]
fn test_struct_1() {
    let mut user = User {
        email: String::from("19951799881@163.com"),
        user_name: String::from("gxc"),
        activate: true,
        sign_in_count: 100,
    };

    println!("new -> {:#?}", user);

    println!("modified -> {:#?}", user);

    let s = String::from("1");
}