/*--------------rust练习---------------*/

fn main() {
    let name:&str = "Rust";

let mut age: i32 = 18;

let isTrue: bool = true;

let scores: [i32; 5] = [100, 90, 80, 70, 60];
println!("Name: {}, Age: {}, IsTrue: {:?}, Scores: {:?}", name, age, isTrue, scores);


fn calculateArea(width: f64, height: f64) -> f64 {
    width * height
}
println!("Area: {}", calculateArea(10.0, 20.0));


let grade: i32 = 90;

let mut result: &str;

 match grade {
    90..=100 => result = "优秀",
    80..=89 => result = "良好",
    70..=79 => result = "中等",
    60..=69 => result = "及格",
    _ => result = "不及格",
 }
 println!("Result: {}", result);


    let result = if grade >= 90 {
        "优秀"
    } else if grade >= 80 {
        "良好"
    } else if grade >= 70 {
        "中等"
    } else {
        "需要努力"
    };
}


