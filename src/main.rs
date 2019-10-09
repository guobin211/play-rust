/// 导入外部库
extern crate rustc_serialize;

/// 使用外部库模块
use rustc_serialize::json;

/// 导入内部库
mod types_mod;
mod user_mod;

/// 内部库模块
use types_mod::types;
use user_mod::user;


/// 定义结构体
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct {
    id: u8,
    name: String,
    books: Vec<String>,
}

/// 启动入口
fn main() {
    // 实例化
    let object = TestStruct {
        id: 1,
        name: "tom".to_string(),
        books: vec!["EN".to_string(), "ZH".to_string()],
    };
    println!("id: {}", object.id);
    println!("name: {}", object.name);
    println!("books: {:?}", object.books);

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();
    println!("{}", encoded);

    // Deserialize using `json::decode`
    let _decoded: TestStruct = json::decode(&encoded).unwrap();
    println!("name: {}", _decoded.name);
}
