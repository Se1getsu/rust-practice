pub fn main01() {
    // 型注釈は省略しても型推論される
    let a: i32 = 5;     // 再代入不可
    let mut b: i32 = 5; // 再代入可
    const CONSTANT: usize = 100; // コンパイルの時点で決まっている定数
    println!("a equals to {}", a);
    println!("b equals to {}", b);
    b = 8;
    println!("b equals to {}", b);
    println!("CONSTANT equals to {}", CONSTANT);

    let y = 5;
    let y = y+1; // シャドーイング
    {
        let y = y*2;
        println!("y equals to {}", y); // 12
    }
    println!("y equals to {}", y); // 6

    // 整数型 i16 = int(16), u16 = uint(16), isize/usize はアーキテクチャによって最適化されたサイズの整数型
    // 整数リテラル 0xff, b'A'
    // 浮動小数点数 f32, f64 ... 現代のCPUではほぼ同じ速度であるので f64 が基準型
    // 真偽値 bool  文字 char  文字列 &str  タプル (i32, char)のように

    let x: usize = 6;
    let y: f64 = 1.5;
    let z = (x as f64) / y; // 型変換
    println!("6 / 1.5 = {}", z);

    // 配列はスタックに確保されるので、要素数は固定
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[4] = {}", a[4]); // 5
    // a[5] を参照するとパニック(強制終了)が起こる

    // Rust特有の代入方法
    let y = {
        let n = 3;
        plus_one(n) * 10 // 文ではないのでセミコロンなし
    };
    println!("y = {}", y); // 40

    // 
    let x = 5;
    if x != 0 {     // != 0 は必要
        println!("x not equals to 0", );
    }

}

fn plus_one(x: i32) -> i32 {
    x + 1  // 文ではないのでセミコロンなし
}
