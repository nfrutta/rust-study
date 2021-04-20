
struct User {
    username: String,
    email: String,
    //sign_in_count: u64,
    //active: bool,
}

fn main() {
    // 構造体のインスタンス生成
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        //active: true,
        //sign_in_count: 1,
    };
    println!("{}:{}", user1.username, user1.email);

    // こういうパターンもある
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        //active: true,
        //sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");
    println!("{}:{}", user2.username, user2.email);
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email, // 同じ名前なので省略できる
//         username, // ここも省略
//         active: true,
//         sign_in_count: 1,
//     }
// }