/// 公共模块 user
pub mod user {
    /// 结构体, 添加debug信息
    #[derive(Debug)]
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }
    /// 添加结构体的实现方法
    impl User {
        /// getter
        pub fn get_name(&self) -> &str {
            // 返回引用
            return self.username.as_str();
        }
        /// setter
        pub fn set_name(&mut self, name: String) {
            self.username = name;
        }
    }

    pub fn run() {
        println!("user is running");
    }

    pub fn create_user(name: String, sign: u64) -> User {
        User {
            username: name,
            email: "admin@11.com".parse().unwrap(),
            sign_in_count: sign,
            active: true
        }
    }
}
