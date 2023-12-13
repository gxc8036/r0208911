#[test]
fn test_ownership() {
    let s = String::from("hello");
    takes_ownership(&s);
    //println!("{}", s);

    let i = 20;
    makes_copy(i);
    println!("outer i is {i}");
}

fn takes_ownership(s: &String) {
    println!("s is {s}");
}

fn makes_copy(i: i32) {
    println!("inner i is {i}");
}

#[test]
fn test_ref() {
    let s = String::from("hello");
    let size = get_len(&s);
    println!("{} lens is {}", s, size);
}

fn get_len(s: &String) -> usize {
    s.len()
}

#[test]
fn test_change() {
    let mut s = String::from("hello");
    change_str(&mut s);
    println!("s is {s}");
}

fn change_str(s: &mut String) {
    s.push_str(", world");
}
