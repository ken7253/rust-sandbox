fn main() {
    // 可変変数（？）の定義
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // 定数の宣言
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS: {}", MAX_POINTS);

    // shadowing
    let y = 5;
    println!("The value of y is: {}", y);

    let y = 6;
    println!("The value of y is: {}", y);

    {
        let y = 10;

        println!("The value of y is: {}", y);
    }
}
