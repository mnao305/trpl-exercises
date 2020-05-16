fn main() {
    println!("Hello, world!");

    // let x = five();
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        // セミコロンを付けるとだめらしい。
        // 暗黙的なreturn? キモイ
        x + 1
    };

    println!("The value of y is: {}", y);

    // これはエラー
    // let x = (let x = 6);

    another_function(5, 6);
}

// fn five() -> i32 {
//     5
// }

fn plus_one(x: i32) -> i32 {
    // 関数は最後の式を暗黙的に返す。
    // セミコロンを付けると式ではなく、ステートメントになるのでエラーになる
    x + 1

    // return使うとこうかな？
    // return x + 1;
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
