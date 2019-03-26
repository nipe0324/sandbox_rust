fn main() {
    println!("Hello, world!");

    // テスト cargo test
    println!("plus: {}", plus(1, 2));

    // エラー処理
    let a : Result<i32,_> = "4693".parse();
    let b : Result<i32,_> = "ha123".parse();
    match a {
        Ok(x) => println!("{}", x),
        Err(_) => println!("変換に失敗")
    };
    match b {
        Ok(x) => println!("{}", x),
        Err(_) => println!("変換に失敗")
    }
}

fn plus(a:i32, b:i32) -> i32 {
    a + b
}

#[test]
fn plus_test() {
    assert_eq!(plus(4, 5), 9);
    assert_eq!(plus(100, -1), 99);
    assert_eq!(plus(114000, 514), 114514);
}
