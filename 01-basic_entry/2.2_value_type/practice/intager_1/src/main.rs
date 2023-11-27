fn main() {
    let x: i32 = 5;
    let y: u32 = 5;

    // assert_eq!(x, y);
    // panic, 类型不同不能进行比较

    let z = 32;
    let v: u32 = 32;
    assert_eq!(z, v); // Success
}
