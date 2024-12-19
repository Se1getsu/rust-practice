pub fn main03() {
    // 【所有権規則】
    // - Rustの各値は、所有者と呼ばれる変数と対応している。
    // - いかなる時も所有者は一つである。
    // - 所有者がスコープから外れたら、値は破棄される。

    // 【ヒープ領域とスタック領域】
    // ヒープは、システムコールによって alloc と free を行う必要のあるメモリ上の領域。低速。
    // スタックは、スタックポインタで管理されるメモリ上の領域。サイズが固定。高速。

    // String はヒープに保存される文字列型
    {
        let mut s = String::from("Hello"); // s がこの値の所有者
        s.push_str(", world!");
        println!("{}", s);
    } // 所有者がスコープから外れて、値は破棄される。

    // コピー, ムーブ, クローン
    movement();

    // 参照, 借用 - 値は同時に 1つの可変参照 か N個の不変な参照 のどちらかしか持てない
    borrowing();

    // ダングリングポインタ
    dangle();
}

fn movement() {
    // 【コピー】 スタックに保存される値は高速にコピーできるので問題なし
    let a = 10;
    let b = a;
    println!("a = {}", a); // OK
    println!("b = {}", b); // OK

    // 【ムーブ】 ヒープに保存される値は所有権のムーブが起こる
    let a = String::from("hoge");
    let b = a;
    // println!("a = {}", a); // NG
    println!("b = {}", b); // OK

    // 【クローン】
    let a = String::from("hoge");
    let b = a.clone();
    println!("a = {}", a); // NG
    println!("b = {}", b); // OK

    // 関数呼び出しでも所有権のムーブが起こる
    let s1 = String::from("piyo");
    let (s2, len) = strlen(s1);
    println!("The length of {} is {}", s2, len);

    fn strlen(text: String) -> (String, usize) {
        let length = text.len();
        (text, length)
    }
}

fn borrowing() {
    // 【借用】
    let s1 = String::from("piyo");
    let len = strlen2(&s1); // 自分の参照を渡している (変更は不可)
    println!("The length of {} is {}", s1, len);

    fn strlen2(s: &String) -> usize {
        s.len()
    }

    // 【可変借用】
    let mut s2 = String::from("piyo");
    push_yon(&mut s2); // Swift でいう inout 引数
    println!("{}", s2); // piyo yon

    fn push_yon(some_string: &mut String) {
        some_string.push_str(" yon");
    }

    // 可変な参照は 1 つしか持てない (データ競合の危険があるから)
    let mut s2 = String::from("piyo");
    let r1: &mut String = &mut s2; // OK
    // let r2: &mut String = &mut s2; // NG
    println!("{}", r1);
    // println!("{}", r2);

    // 不変な借用が存在する間は、可変借用できない
    let r1: &String = &s2; // OK
    // let r2: &mut String = &mut s2; // NG
    println!("{}", r1);
    // println!("{}", r2);
}


fn dangle() {
    // ダングリングポインタ: 解放されて無効になったポインタが参照されてしまうこと。

    // let s1: &String = dangle_ng();
    // println!("{}", s1);
    //
    // fn dangle_ng() -> &String {
    //     // s は関数内で定義されているので関数内でのみ有効
    //     let s = String::from("hogera");
    //     &s   // 所有者を失った値の参照が返される
    // }

    let s2: String = dangle_ok();
    println!("{}", s2);

    fn dangle_ok() -> String {
        let s = String::from("hogera");
        s   // 所有権がムーブされるので問題なし
    }
}


