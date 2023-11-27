fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    let c: u8 = a + 20;
    println!("{}", b); // 19
    println!("{}", c);
}
