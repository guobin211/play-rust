// 模块
mod sound;

// 命名空间
use std::io;
use std::collections::{hash_map, HashMap};
use crate::sound::instrument::{calculate_length, change};

// 结构体
struct Rectangle {
    width: u32,
    height: u32,
}

// 方法体
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

enum IpAddKind {
    V4,
    V6,
}

fn ip_in_enum(kind: IpAddKind) -> u32 {
    match kind {
        IpAddKind::V4 => 1,
        IpAddKind::V6 => 2
    }
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut jack = User {
        username: String::from("jack"),
        email: String::from("211@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    jack.sign_in_count = 10;

    println!("count: {}", jack.sign_in_count);

    let mut hello = String::from("hello world");
    println!("length: {}", calculate_length(&hello));
    // 修改string
    change(&mut hello);
    let res= first_word(&hello);
    println!("slice: {}", res);
    // 使用模块
    sound::instrument::clarinet();
    // Vector
    let mut arr: Vec<i32> = Vec::new();
    arr.push(1);
    arr.push(2);
    arr.push(1);
    println!("arr length: {}", arr.len());
    for el in &arr {
        println!("element: {}", el);
    }
    // HashMap
    let mut hash: HashMap<String, u32> = HashMap::new();
    hash.insert("jack".parse().unwrap(), 22);
    hash.insert(String::from("tom"), 18);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
    // Struct结构体
    let mut rect = Rectangle { width: 20, height: 10 };
    println!("rect: {}", area(&rect));
    rect.width = 10;
    println!("rect: {}", rect.area());
    let localhost = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{}", localhost.address);

    types();
    guest();
}
// 字符串截取
fn first_word(s: &String) ->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}

fn types() {
    // 私有变量
    let _guess: u32 = "42".parse().expect("Not a number!");
    let big: bool = _guess > 22;
    println!("{}", big);
}

fn guest() {
    println!("猜一猜数字");
    println!("请输入数字");
    // 不可变
    let default = 600;
    const MAX_POINT: u32 = 100_000;
    println!("compare: {}", MAX_POINT > default);
    // 可变
    let mut guest = String::new();
    io::stdin().read_line(&mut guest).expect("Failed to read line");

    println!("You guessed: {}", guest);
    println!("default: {} and your: {}", default, guest)
}
