// ❤️❤️ 4. fix error

// 修复错误
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}
