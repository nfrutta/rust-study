fn main() {
    println!("Hello, world!");

    another_function();

    another_function2(5);

    another_function3(5, 6);

    let x = five();
    println!("The value of x is: {}", x);
}

// 関数名はスネークケース
fn another_function() {
    println!("Another function.");
}

// 引数1つ
fn another_function2(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}

// 引数2つ
fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 戻り値あり
fn five() -> i32 {
    5 // セミコロンは不要
}