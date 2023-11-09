// mod logo;
// mod hello;

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界你好";
    let russian = "Здравствуй, мир";
    let regions = [southern_germany, chinese, russian];
    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    // logo::logo();
    // hello::hello()
    greet_world();
    println!("Hello, world!");
}
