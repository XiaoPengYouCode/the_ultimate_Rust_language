// ❤️❤️ 9. 你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x,y], __);
// }

fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
}

// _ 表示忽略对应位置的一个，..表示忽略对应位置的所有
