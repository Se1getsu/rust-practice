pub fn main06() {
    // ベクタ
    vector();
}

/**
 * ベクタ Vec<Type> は配列 [Type; length] と違って
 * 要素数が可変な動的配列。
 * 色んな型のデータを入れたい場合は Enum で1つの型にまとめる。
 */
fn vector() {
    // Vec::new で生成する方法
    // let v: Vec<i32> = {
    //     let mut v = Vec::new();
    //     v.push(1);
    //     v.push(2);
    //     v.push(3);
    //     v
    // };
    // vec!マクロ で生成する方法
    let mut v: Vec<i32> = vec![1, 2, 3];

    // 要素の参照
    let e = &v[1]; // &v[5] はパニック
    println!("v[1] = {}", e);
    let e = v.get(1); // &v[5] は None
    println!("v[1] = {:?}", e);

    // for 文での利用
    for i in &v {
        println!("{}", i);
    }
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }
    for i in &mut v {
        *i += 30;
    }
}
