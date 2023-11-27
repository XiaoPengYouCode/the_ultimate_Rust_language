// 使用以下方法来修复编译器输出的 warning :
// ❤️ 1. 使用一种方法
// ❤️❤️ 2. 使用两种方法

// fn main() {
//     let x = 1;
// }

// 1. approach 1
// fn main() {
//     let _x = 1;
// }

// 2. approach 2
fn main() {
    let x = 1;
    println!("{}", x);
}
