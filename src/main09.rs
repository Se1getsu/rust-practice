use std::{fs::{remove_file, File}, io::{self, ErrorKind, Read}};

pub fn main09() {
    // 回復不能なエラー: panic!
    learn_panic();
    // 回復可能なエラー: Result
    learn_result();
}

fn learn_panic() {
    if false {
        // パニックが起こる簡単な例
        let v = vec![1, 2, 3];
        v[99];
    }
    if false {
        // パニックを起こすプログラム
        panic!("Hello, panic!");
    }
}

fn learn_result() {
    let f: Result<File, io::Error> = File::open("whafoo.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("whafoo.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Failed to create file: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("Failed to open file: {:?}", error)
        },
    };
    println!("{:?}", f);

    // 以下の unwrap(), expect(), ?演算子 は Option/Result どちらでも使える

    // Swift: x! (Forced Unwrapping)
    // Rust: x.unwrap()
    let _f: File = File::open("whafoo.txt").unwrap();

    // Swift: guard let x else { fatalError("...") }
    // Rust: x.expect("...")
    let _f: File = File::open("whafoo.txt")
                    .expect("Failed to open file");

    // Swift: guard let x else { return nil }
    // Rust: x?
    fn read_whafoo() -> Result<String, io::Error> {
        let mut s = String::new();
        // 失敗して io::Error が返ってきたら Err(e) を返す
        File::open("whafoo.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    let res = read_whafoo();
    println!("{:?}", res);

    // (後処理) 作成されたファイルを削除
    let _ = remove_file("whafoo.txt").unwrap();
}
