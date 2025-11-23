#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
    
    // Getter 方法：获取姓名
    pub fn name(&self) -> &String {
        &self.name
    }
    
    // Getter 方法：获取年龄
    pub fn age(&self) -> u32 {
        self.age
    }
}