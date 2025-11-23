// main.rs - 主程序
// 假设 crate 名称为 langshift-project
use rust_module::math::{add, multiply, PI, Calculator};
use rust_module::utils::format_number;
use rust_module::User;

fn main() {
    println!("PI = {}", PI); // PI = 3.14159
    println!("add(5, 3) = {}", add(5, 3)); // add(5, 3) = 8
    println!("multiply(4, 2) = {}", multiply(4, 2)); // multiply(4, 2) = 8
    
    let mut calc = Calculator::new();
    calc.add(10).add(5);
    println!("calc result = {}", calc.get_result()); // calc result = 15
    
    println!("formatted: {}", format_number(3.14159)); // formatted: 3.14
    
    // 使用 User 结构体
    let user = User::new("张三".to_string(), 25);
    println!("用户信息: {:?}", user); // 使用 Debug trait 打印
    println!("姓名: {}, 年龄: {}", user.name(), user.age());
    
    // 创建多个用户
    let user2 = User::new("李四".to_string(), 30);
    println!("第二个用户: {:?}", user2);
}