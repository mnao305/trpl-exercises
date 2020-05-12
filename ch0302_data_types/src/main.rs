fn main() {
    // 型注釈をつけないとエラーになるという話
    let guess: u32 = "42".parse().expect("error");
    println!("guess is {}", guess);

    // スカラー型 単一の値を表しているらしい
    // 整数とか小数点とか真偽値とか文字

    // Integer Types
    // 整数型でも色々ビットによって種類がある
    // i付きだと符号付、uだと符号なし
    // 整数はHexやらBinaryやらいろんな書き方がある。
    // 10_000←こんな風にアンスコで見やすくできるのが強いと思う
    let int = 10_000;
    println!("int is {}", int);

    // Floating-Point Types
    // 小数にもf64とf32と種類がある。精度が高いからf64がデフォらしい
    let float = 3.14;
    println!("float is {}", float);

    // Numeric Operations
    // 基本的な数値演算はできるという話
    // 足し算
    let _sum = 5 + 10;
    // 引き算
    let _difference = 95.5 - 4.3;
    // 掛け算
    let _product = 4 * 30;
    // 割り算
    let _quotient = 56.7 / 32.2;
    // 余り計算
    let _remainder = 43 % 5;

    // 関係ないけど、変数名の先頭にアンスコ刺すと未使用の警告が出なくなるんだね。

    // The Boolean Type
    // 真偽値
    let _t = true;

    let _f: bool = false;

    // The Character Type
    // 文字
    // 文字列リテラルはダブルクォーテーションだけど、文字リテラルはシングルクォーテーションを使う。気を付けよう
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    let _japanese = '字';

    // Compound Types
    // 何これ
    // 複合型 タプルと配列のことらしい

    // The Tuple Type
    // 複数の型の値を1つにグループ化する
    // 長さは固定されているから後から増やしたり減らしたりはできないらしい。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // ↓こんな感じで分解できるぜ
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // ピリオド付けた後に何番目か指定すれば値を取れる
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // The Array Type
    // 配列はすべて同じ型じゃないといけない。長さは固定
    let a = [1, 2, 3 ,4 ,5 ];
    // 配列と似たようなものでベクトル型というものがあるらしい。これはサイズを変えられる。
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // ↑こういう月の名前とか、長さが変わらないやろってやつは配列でOKって感じか。
    let _a_type: [i32; 5] = [1, 2, 3, 4, 5];
    let _ary = [3; 5];
    // ↑初期化の仕方。[初期値;長さ]という感じ
    let _first = a[0];
    let _second = a[1];
    // 配列要素の取り方はよくある感じだ。

    // ↓長さを超えたindexでアクセスしようとすると実行時に落ちる
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
}
