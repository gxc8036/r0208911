#[test]
fn test_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("==> {} {}", hello, world);
}

#[test]
fn test_slice_1() {
    let s = String::from("1中1国A人");

    for c in s.chars() {
        println!("{}", c);
    }
}

#[test]
fn test_first_word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the word is {}", word);
    s.clear();
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

#[test]
fn test_str() {
    let s1 = String::from("hello world");
    let h = &s1[..2];
    println!("{}", h);
}

#[test]
fn test_str2() {
    let mut s1 = String::from("123");
    s1.push('4');
    println!("{}", s1);

    let mut s2 = String::from("abc");
    s2.insert_str(0, "1");
    println!("{}", s2);
}

#[test]
fn test_str3() {
    let s = String::from("hello rust , learn rust is my favorite!");
    let s1 = s.replace("rust", "RUST");
    dbg!(s1);
}

#[test]
fn test_str4() {
    let s1 = "hello";
    let s2 = "rust";

    let s = format!("{},{}", s1, s2);
    println!("{s}")
}

#[test]
fn test_str5() {
    let longer_delimiter = r####"A string "" """""""\n with "# in it. And even "##!"####;
    println!("{}", longer_delimiter);
}

#[test]
fn test_str6() {
    let mut bytes:Vec<u8> = Vec::new();

    for b in "Aaおはようございます，你好".bytes() {
        bytes.push(b);
    }
    println!("{:?}",bytes);

    let s = String::from_utf8(bytes);
    println!();

    match s {
        Ok(v) => println!("ok: {}", v),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    }
}

#[test]
fn test_str7() {
    for i in 1..=200 {
        let bytes = vec![i];
        let s = String::from_utf8(bytes);
        match s {
            Ok(v) => {
                if i <= 31 || i == 127 {
                    println!("{}: Control character", i);
                } else {
                    println!("{}: {}", i, v);
                }
            }
            Err(_) => println!("{}: Invalid UTF-8 sequence", i),
        }
    }
}
