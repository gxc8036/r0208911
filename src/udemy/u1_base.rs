use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Instant;

#[test]
fn test_string_1() {
    let s = "hello 你好啊！";
    let mut vec = Vec::new();
    for x in s.bytes() {
        vec.push(x);
    }

    println!("s bytes is {:?}", vec);

    let res = String::from_utf8(vec);

    match res {
        Ok(v) => println!("s decode is {:?}", v),
        Err(err) => println!("decode fail: {:?}", err),
    }

    println!("finished");
}

#[test]
fn test_string_2() {
    let s = "hello 你好啊！";
    for x in s.chars() {
        println!("{:?}", x);
    }
}

#[test]
fn test_ref_1() {
    let mut s1 = String::from("hello");
    do_stuff(&mut s1);
    println!("{}", s1);
}

fn do_stuff(s: &mut String) {
    *s = "gus".to_string();
    s.push_str(",hello");
}

#[test]
fn test_ref_2() {
    let mut i = 21;
    do_stuff_2(&mut i);
    println!("{}", i);
}

fn do_stuff_2(s: &mut i32) {
    *s = 10;
}

#[test]
fn test_thread_01() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let start = Instant::now();
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("--------------------------------------------------------");
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    let start = Instant::now();
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    println!("Result: {}", counter.load(Ordering::Relaxed));
    println!("Time elapsed: {:?}", duration);
}

#[derive(Debug)]
struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    fn moves(&self) {
        println!("{}", self.life);
    }

    fn boo() {
        println!("---");
    }
}

#[test]
fn test_struct() {
    let mut fox = RedFox::new();
    fox.life = 100;
    println!("{:#?}", fox);
    fox.moves();
    RedFox::boo();
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str { "u8-x" }
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str { "fox" }
}


fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise())
}

#[test]
fn test_impl_t() {
    let fox = RedFox::new();
    print_noise(fox);
    print_noise(15_u8);
}