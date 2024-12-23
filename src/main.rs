use main01::main01;
use main02::main02;
use main03::main03;
use main04::main04;
use main05::main05;
use main06::main06;
use main07::main07;
use main08::main08;
use main09::main09;
use main10::main10;

mod main01;
mod main02;
mod main03;
mod main04;
mod main05;
mod main06;
mod main07;
mod main08;
mod main09;
mod main10;

fn main() {
    // 変数と型
    main01();
    // 制御構文
    main02();
    // 所有権
    main03();
    // 構造体
    main04();
    // 列挙型
    main05();
    // ベクタ
    main06();
    // 文字列
    main07();
    // ハッシュマップ
    main08();
    // panic!, Result
    main09();
    // ジェネリクス, トレイト
    main10();
}
