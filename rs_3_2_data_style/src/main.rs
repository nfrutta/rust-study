fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // 整数型
    let a = 5;
    print_typename(a); // i32
    // 整数値型は基本i32を使う

    // 浮動小数点型
    let x = 2.0;
    let y: f32 = 3.0;
    print_typename(x); // f64
    print_typename(y); // f32

    // 数値の演算
    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    // 掛け算
    let product = 4 * 30;
    // 割り算
    let quotient = 56.7 / 32.2;
    // 余り
    let remainder = 43 % 5;

    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);

    let t = true;
    let f: bool = false; // 明示的型注釈付きで
    print_typename(t);
    print_typename(f);

    // タプル
    // 型指定で宣言
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // なしでも定義できる
    let tup = (500, 6.4, 1);
    // タプルの取り出し（分解）
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // ピリオド数字でアクセスも可能
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{},{},{}", five_hundred, six_point_four, one);

    // 配列(Rustでは固定長)
    let a = [1, 2, 3, 4, 5];
    // 要素数が分かっている場合、例えば月の定義とかに使う
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // 要素の取り出しは他の言語と同じ
    println!("{}", months[4]);
}
