
fn main() {
     /*--------------------------------变量声明与作用域--------------------------------*/
    // Rust 变量声明
    let immutable_var = "默认不可变";
    let mut mutable_var = "可以重新赋值"; // 需要 mut 关键字

    // 可变变量可以重新赋值
    mutable_var = "新的值";
    println!("{}", mutable_var); // 输出: 新的值

    // 不可变变量不能重新赋值
    // immutable_var = "尝试修改"; // 编译错误！

    // 变量遮蔽（shadowing）- Rust 特有概念
    let x = 5;
    let x = x + 1; // 创建新的变量 x，遮蔽原来的 x
    println!("x = {}", x); // 输出: x = 6

     /*--------------------------------基本数据类型--------------------------------*/
     // Rust 是静态类型语言，类型在编译时确定
    // Rust 基本数据类型
    let integer: i32 = 42;           // 32位有符号整数
    let unsigned: u32 = 42;          // 32位无符号整数
    let float: f64 = 3.14;           // 64位浮点数
    let boolean: bool = true;        // 布尔值
    let character: char = 'A';       // Unicode 字符
    let mut string1: String = String::from("Hello"); // 字符串
    let mut string_slice: &str = "World"; // 字符串切片
    string1 = String::from("World2"); // 可以重新赋值

    // 数组 - 固定长度
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // 元组 - 可以包含不同类型
    let tuple: (i32, f64, &str) = (500, 6.4, "hello");

    // 访问数组和元组元素
    println!("数组第一个元素: {}", array[0]);
    println!("元组第一个元素: {}", tuple.0);
    println!("字符串切片: {}", string1);

    /*--------------------------------控制流--------------------------------*/

     // Rust 条件语句
     let age = 18;

     if age >= 18 {
         println!("成年人");
     } else if age >= 12 {
         println!("青少年");
     } else {
         println!("儿童");
     }
 
     // if 是表达式，可以赋值
     let status = if age >= 18 { "成年人" } else { "未成年人" };
     println!("状态: {}", status);
 
     // match 语句（类似 switch，但更强大）
     let day = "Monday";
     match day {
         "Monday" => println!("星期一"),
         "Tuesday" => println!("星期二"),
         _ => println!("其他日子"), // _ 是通配符
     }
 
     // 循环
     for i in 0..5 {
         println!("{}", i);
     }
 
     let numbers = vec![1, 2, 3, 4, 5];
     let nums = [1,3];
     for num in &numbers {
         println!("{}", num);
     }
     for count in nums {
        println!("{}", count);
     }
     println!("nums: {:?}", nums);
//    for num in numbers  {  // for num in numbers：移动所有权，循环后 numbers 无效。
//      println!("{}", num);
//    }
   println!("numbers: {:?}", numbers);
     // while 循环
     let mut count = 0;
     while count < 5 {
         println!("count: {}", count);
         count += 1;
     }
 
     // loop 循环（无限循环）
     let mut counter = 0;
     loop {
         counter += 1;
         if counter > 5 {
             break; // 跳出循环
         }
         println!("counter: {}", counter);
     }
    
     /*--------------------------------函数--------------------------------*/
      // Rust 函数定义
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    // 函数调用
    println!("{}", greet("Rust")); // Hello, Rust!

    // 默认参数（通过 Option 实现）
    fn greet_with_default(name: Option<&str>) -> String {
        let name = name.unwrap_or("World");
        format!("Hello, {}!", name)
    }

    println!("{}", greet_with_default(None));        // Hello, World!
    println!("{}", greet_with_default(Some("Rust"))); // Hello, Rust!

    // 多个参数
    fn add(a: i32, b: i32) -> i32 {
        a + b // 最后一个表达式是返回值，不需要 return
    }

    fn print_message(message: Option<&str>) {
        let default_message = "default";
        let message = message.unwrap_or(default_message);
        println!("{}", message);
    }

    print_message(None);
    print_message(Some("Rust"));

    println!("{}", add(5, 3)); // 8

    // 函数作为表达式
    fn get_status(age: i32) -> &'static str {
        if age >= 18 {
            "成年人"
        } else {
            "未成年人"
        }
    }

    println!("{}", get_status(20)); // 成年人


    /*--------------------------------闭包--------------------------------*/
   
    fn main() {
        // Rust 闭包
        fn create_counter() -> impl FnMut() -> i32 {
            let mut count = 0;
            move || {
                count += 1;
                count
            }
        }
    
        let mut counter = create_counter();
        println!("{}", counter()); // 1
        println!("{}", counter()); // 2
        println!("{}", counter()); // 3
    
        // 闭包语法
        let add = |a, b| a + b;
        println!("{}", add(5, 3)); // 8
    
        // 捕获外部变量
        let multiplier = 2;
        let multiply = |x| x * multiplier;
        println!("{}", multiply(5)); // 10
    
        // 闭包类型注解
        let expensive_closure = |num: u32| -> u32 {
            println!("计算中...");
            num * 2
        };
    
        println!("{}", expensive_closure(5)); // 计算中... 10
    
        // 迭代器中使用闭包
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        println!("{:?}", doubled); // [2, 4, 6, 8, 10]
    }

}


