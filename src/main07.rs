pub fn main07() {
    // 文字列
    learn_string()
}

fn learn_string() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    s1.push('!');
    println!("{}", s1);  // Hello!

    // 連結: String + &str = String
    // s1 から s2 に所有権が移る
    let s2 = s1 + " World!";
    // println!("{}", s1);  // Error
    println!("{}", s2);  // Hello! World!

    // format! マクロ
    let s3 = format!("{} {}", s2, "Everyone!");
    println!("{}", s2);  // Hello! World!
    println!("{}", s3);  // Hello! World! Everyone!

    // 添え字アクセスは不可, スライス
    let s = String::from("ほげ");
    // // Pyhonの s[0]: 書記素クラスタに分けた時の 1 文字目
    // // Cの s[0]: 1 バイト目
    // // Rustの s[0]: 何を文字とするかが明確でないので、コンパイルエラー
    // let fitst = &s3[0];
    //
    // // スライスは可能(バイト単位)
    // // ただし、書記素クラスタをまたぐとパニックが起こる
    // let first = &s[..2];
    // // byte index 2 is not a char boundary; it is inside 'ほ' (bytes 0..3) of `ほげ`

    for c in s.chars() {
        println!("c: {}", c);   // ほ, げ
    }
    for b in s.bytes() {
        println!("b: {:X}", b); // E3, 81, BB, E3, 81, 92
    }
}
