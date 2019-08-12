// 声明引入模块
//mod types_mod;
mod user_model;

// 使用命名空间
//use types_mod::types;
use user_model::user;

// 启动入口
fn main() {
    user::run();
    let name = "jack";
    let mut user1 = user::create_user((name).parse().unwrap(), 22);
    println!("user: {:?}", user1);
    user1.set_name(String::from("tom"));
    println!("user: {:?}", user1);
    println!("user name: {}", user1.get_name());
}
