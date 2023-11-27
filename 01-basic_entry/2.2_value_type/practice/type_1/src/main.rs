// 这个例子感觉还挺好玩的

fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
