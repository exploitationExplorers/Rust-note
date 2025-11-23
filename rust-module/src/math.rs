// math.rs - 数学模块
pub const PI: f64 = 3.14159;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 结构体（类似 JavaScript 的类）
pub struct Calculator {
    result: i32,
}

impl Calculator {
    // 构造函数
    pub fn new() -> Self {
        Calculator { result: 0 }
    }
    
    pub fn add(&mut self, x: i32) -> &mut Self {
        self.result += x;
        self
    }
    
    pub fn get_result(&self) -> i32 {
        self.result
    }
}