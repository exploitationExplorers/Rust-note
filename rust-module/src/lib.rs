// lib.rs - 主模块文件
pub mod math; // 声明 math 模块（公开）
pub mod utils; // 声明 utils 模块（公开）
pub mod models; // 声明 models 模块（公开）
// 重新导出模块内容
pub use math::{add, multiply, PI, Calculator};
pub use utils::format_number;
pub use models::User;




