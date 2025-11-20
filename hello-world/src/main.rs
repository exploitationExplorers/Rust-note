// src/main.rs
fn main() {
    println!("Hello, Rust!");
    
    // 变量声明（不可变）
    let name = "LangShift";
    println!("Hello, {}!", name);
    
    // 函数定义
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    println!("{}", greet("Developer"));

    // Rust: 变量默认不可变
  let name = "Rust";
  // name = "R"; // 错误：变量默认不可变
  println!("{}", name);

  // 使用 mut 关键字声明可变变量
  let mut age = 25;
  age = 26; // 可以重新赋值
  println!("{}", age);
}