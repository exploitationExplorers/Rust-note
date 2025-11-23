// // Rust 所有权系统示例
// fn create_user() {
//     let user = User {
//         name: String::from("Alice"),
//         age: 25,
//         preferences: Preferences {
//             theme: String::from("dark"),
//             language: String::from("zh-CN"),
//         },
//     };
    
//     // 所有权转移：user 的所有权移动到 user_ref
//     let user_ref = user;
    
//     // 这行代码会编译错误！user 已经被移动
//     // println!("{}", user.name); // 错误：use of moved value
    
//     // 可以修改 user_ref
//     let mut user_ref = user_ref;
//     user_ref.age = 26;
//     println!("{}", user_ref.age); // 26
    
//     // 创建新的 User
//     let new_user = User {
//         name: String::from("Bob"),
//         age: 30,
//         preferences: Preferences {
//             theme: String::from("light"),
//             language: String::from("en-US"),
//         },
//     };
    
//     // user_ref 指向新对象
//     let user_ref = new_user;
//     println!("{}", user_ref.name); // "Bob"
// }

// // 定义结构体
// struct User {
//     name: String,
//     age: u32,
//     preferences: Preferences,
// }

// struct Preferences {
//     theme: String,
//     language: String,
// }

// // 向量（类似 JavaScript 数组）的所有权
// fn vector_ownership() {
//     let mut numbers = vec![1, 2, 3];
//     let numbers_ref = &mut numbers; // 可变借用
    
//     numbers_ref.push(4);
//     println!("{:?}", numbers); // [1, 2, 3, 4]
    
//     // 创建深拷贝
//     let deep_copy = numbers.clone();
//     println!("{:?}", deep_copy); // [1, 2, 3, 4]
    
//     // 修改原数组
//     numbers.push(5);
//     println!("{:?}", numbers); // [1, 2, 3, 4, 5]
//     println!("{:?}", deep_copy); // [1, 2, 3, 4] - 深拷贝不变
// }

// fn main() {
//     create_user();
//     vector_ownership();
// }

/**--------------------------------所有权规则--------------------------------*/


// 规则 1: 每个值都有一个所有者
// Rust 中的数据分为两种：
// 1. 拥有所有权的数据 (如 String, Vec): 默认行为是“移动” (move)。当所有权转移后，原变量失效。
// 2. 实现了 Copy Trait 的数据 (如整数、浮点数、布尔值、字符、固定大小数组): 默认行为是“复制” (copy)。复制后，原变量仍然有效。
// fn rule_one() {
//     // String 类型：默认行为是移动
//     let s1 = String::from("hello"); // s1 是所有者
//     let s2 = s1; // 所有权从 s1 移动到 s2
//     // println!("{}", s1); // 错误：s1 已经被移动，不能再使用
//     println!("{}", s2); // 正确：s2 现在是所有者

//     // i32 类型：实现了 Copy Trait，默认行为是复制
//     let x = 5; // x 是所有者
//     let y = x; // x 的值被复制给 y，x 仍然有效
//     println!("x = {}, y = {}", x, y); // 正确：x 和 y 都可以使用
// }

// // 规则 2: 同一时间只能有一个所有者
// fn rule_two() {
//     let s1 = String::from("hello");
//     let s2 = s1; // s1 的所有权移动到 s2
//     // let s3 = s1; // 错误：s1 已经被移动
//     let s3 = s2; // 正确：s2 的所有权移动到 s3
// }

// // 规则 3: 当所有者离开作用域时，值被丢弃
// fn rule_three() {
//     {
//         let s = String::from("hello"); // s 进入作用域
//         // 使用 s
//         println!("{}", s);
//     } // s 离开作用域，内存被释放
//     // println!("{}", s); // 错误：s 已经被释放
// }

// // 函数中的所有权
// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// } // some_string 离开作用域，内存被释放

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// } // some_integer 离开作用域，但 i32 是 Copy 类型，不需要释放


// // 不可变借用（类似 JavaScript 的只读引用）
// fn calculate_length(s: &String) -> usize {
//     s.len() // 返回字符串长度
// } // s 离开作用域，但因为它只是借用，不会释放内存

// // 可变借用（类似 JavaScript 的可写引用）
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// // 悬垂引用的例子（这会导致编译错误）
// /*
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 错误：返回对局部变量的引用
// } // s 离开作用域，内存被释放，但返回了对它的引用
// */

// // 正确的做法
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s // 返回所有权，而不是引用
// }

// fn main() {
//     rule_one();
//     rule_two();
//     rule_three();
    
//     let s = String::from("hello");
//     take_ownership(s); // s 的所有权移动到函数中
//     // println!("{}", s); // 错误：s 已经被移动
    
//     let x = 5;
//     make_copy(x); // x 被复制到函数中
//     println!("{}", x); // 正确：x 仍然有效
// }


/*--------------------------------生命周期--------------------------------*/

// 生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
    
    // 生命周期注解确保返回的引用在参数的生命周期内有效
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("{}", i.part);
}

// 生命周期省略规则
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 上面的函数等同于：
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    // 函数体相同
    s
}