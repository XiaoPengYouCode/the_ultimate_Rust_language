fn hello() {
    let chinese = "你好，中国";
    let english = "Hello, World!";
    let southern_germany = "Grüß Gott!";
    let regions = [chinese, english, southern_germany];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    hello();
}
