/**
* sound 模块
*
* @author GuoBin on 2019-07-23
*/

pub mod instrument {
    pub fn clarinet() {
        println!("sound file")
    }
    // 一个不可变String引用
    pub fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // &mut String 一个可变String引用
    pub fn change(s: &mut String) {
        s.push_str("add string")
    }
}
