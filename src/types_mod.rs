/// types 公共模块
/// run() 启动方法
/// number_size() 私有方法
pub mod types {
    pub fn run() {
        println!("hello rust types");
        number_size();
        number_string();
        tuple();
    }
    // 数字类型
    fn number_size() {
        let guest: u32 = "42".parse().expect("Not a number!");
        let count: i32 = -1000000;
        let amount: u64 = 1000000;
        let rich_number: usize = 222;
        let res = guest + amount as u32;
        println!("guest + amount: {}", res);
        println!("count: {}", count + 1000001);
        println!("长度可变: {}", rich_number);
        println!("int to string: {}", guest.to_string());
    }
    // 数字转换
    fn number_string() {
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';

        println!("decimal: {}", decimal);
        println!("hex 16进制: {}", hex);
        println!("octal: {}", octal);
        println!("binary 二进制: {}", binary);
        println!("byte 字节: {}", byte);
        // 私有属性
        let _res = 9 % 5;
        println!("数学运算取余数 9 % 5：{}", _res);
    }
    // 元祖类型
    fn tuple() {
        let tup: (i32, f32, u8) = (10, 5.5, 1);
        let (x, y, z) = tup;
        println!("x: {}, y: {}, z: {}", x, y, z);
        let _person = "mary";
        let arr = ["jack", "tom"];
        for person in arr.iter() {
            println!("{}", person);
        }
        println!("arr length: {}", arr.len());
        let mut vec: Vec<i32> = Vec::new();
        for i in 0..3 {
            vec.push(i + 1);
        }
        println!("index 0: {}", vec[0])
    }
}
