// constは型を必ず書く
// 数値は_で見やすくできる。
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("MAX_POINTS is: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // シャドーイングで型違いでも入れられる。すごい
    let spaces = "       ";
    let spaces = spaces.len();
    println!("spaces is: {}", spaces);
    // ↓これだとエラーになる
    // let mut spaces = "       ";
    // spaces = spaces.len();
}
