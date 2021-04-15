fn main() {
    /* 変数定義 */

    // Rustの変数は基本不変で定義される
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; // こいつはエラーになる
    //println!("The value of x is: {}", x);
    // 再代入を許可する場合はmutをつける
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /* 定数 */
    const MAX_POINTS: u32 = 100_000;
    println!("const value: {}", MAX_POINTS);

    /* シャドーイング */
    // 通常letで定義した変数へ再代入はできないが
    let x = 5;
    // 以下の場合は再定義になるので（前に定義された変数を覆い隠すというらしい）エラーにならない
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // 同じ変数名で再定義できる（再利用できる）
    // ちなみにシャドーイングすると型も変えることができる
    let spaces = "   ";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces: {}", spaces)
    // 当然以下みたいな場合はビルドエラーになる
    //let mut spaces = "   ";
    //spaces = spaces.len();
}
