use main01::main01;
use main02::main02;
use main03::main03;
use main04::main04;
use main05::main05;

mod main01;
mod main02;
mod main03;
mod main04;
mod main05;

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
}
