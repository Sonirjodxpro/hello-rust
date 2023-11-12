fn main() {
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);

    // let a: u8 = 255;

    //使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    // let b = a.wrapping_add(30);
    // println!("a = {}, b = {}", a, val);

    //如果使用 checked_* 方法时发生溢出，则返回 None 值
    // let b = a.checked_add(30);
    // match b {
    //     Some(val) => println!("a = {}, b = {}", a, val),
    //     None => println!("Overflow occurred"),
    // }

    //使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // let b = a.overflowing_add(30);
    // println!("a = {}, b = {},b.1 = {}", a, b.0, b.1);

    // 使用 saturating_* 方法使值达到最小值或最大值

    // let b = a.saturating_add(30);
    // println!("a = {}, b = {}", a, b);

    assert!(0.1 + 0.2 == 0.3);
}
