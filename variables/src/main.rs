fn main() {
    // https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html#%E5%A4%89%E6%95%B0%E3%81%A8%E5%8F%AF%E5%A4%89%E6%80%A7
    // 可変変数（？）の定義
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    // 定数の宣言
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS: {}", MAX_POINTS);

    // shadowing
    // https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html#%E3%82%B7%E3%83%A3%E3%83%89%E3%83%BC%E3%82%A4%E3%83%B3%E3%82%B0
    let y = 5;
    // y => 5
    println!("The value of y is: {}", y);
    // y => 6
    let y = 6;

    {
        let y = 10;
        // y => 10
        println!("The value of y is: {}", y);
    }
    // y => 6
    println!("The value of y is: {}", y);
}
