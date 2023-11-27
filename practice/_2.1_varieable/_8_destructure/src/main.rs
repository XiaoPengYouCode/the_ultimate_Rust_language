// ❤️❤️ 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
// // 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
