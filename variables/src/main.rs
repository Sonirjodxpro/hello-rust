fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // _开头会忽略未使用变量
    // let _x = 5;
    // let _y = 10;

    // 解构 有点懵逼，先跳过 主要是()和[]的用法
    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a = {:?},b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    // struct Struct {
    //     e: i32,
    // }
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _, _] = [1, 2, 3, 4, 5];

    // println!("a = {:?},b = {:?},c = {:?},d = {:?}", a, b, c, d);

    // Struct { e, .. } = Struct { e: 5 };

    // assert_eq!([1, 2, 1, 3, 5], [a, b, c, d, e]);

    // 常量 _ 可以用作数字分隔符 100_000 == 100000;
    // const MAX_POINTS: u32 = 100_000;

    // 变量遮蔽 其实跟js中的局部作用域差不饿多
    // let x = 5;
    // println!("The value of x is: {}", x);

    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // let space = "    ";
    // let spaces = space.len();

    // let mut spaces = "      ";
    // spaces = spaces.len();

    // 变量绑定与结构联系
    // 1.
    // let x: i32 = 1; // 未初始化，但被使用
    // let _y: i32; // 未初始化，也未被使用
    // println!("x is equal to {}", x);

    // 2.
    // let mut x = 1;
    // x += 2;

    // println!("x = {}", x);

    // 3.
    // let x: i32 = 10;
    // let y: i32 = 5;
    // {
    //     println!("x 的值是 {}, y 的值是 {}", x, y);
    // }
    // println!("x 的值是 {}, y 的值是 {}", x, y);

    //4.
    // define_x();

    // 5.
    // let x: i32 = 5;
    // {
    //     let x = 12;
    //     assert_eq!(x, 12);
    // }

    // assert_eq!(x, 5);

    // let x = 42;
    // println!("{}", x); // 输出 "42".

    // 6.
    // let mut x: i32 = 1;
    // x = 7;
    // // 遮蔽且再次绑定
    // let mut x = x;
    // x += 3;

    // let y = 4;
    // // 遮蔽
    // let y = "I can also be bound to text!";

    // 7.
    // let _x = 1;

    // 8.
    // let (mut x, y) = (1, 2);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    // 9.
    // let (x, y);
    // (x, ..) = (3, 4);
    // [.., y] = [1, 2];
    // // 填空，让代码工作
    // assert_eq!([x, y], [3, 2]);
}
// fn define_x() {
//     let x = "hello";
//     println!("{}, world", x);
// }
