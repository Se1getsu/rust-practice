// 標準ライブラリでは Enum を使って IPv4/IPv6 が定義されている

// struct Ipv4Addr {
//     ...
// }
// struct Ipv6Addr {
//     ...
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

pub fn main05() {
    // データを紐付ける (データを含む列挙子)
    data_associating();

    // Option 型
    optional();

    // match 文
    match_stmt();
    // if let 記法
    if_let();
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名構造体
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn data_associating() {
    let mut message: Message;

    message = Message::Quit;
    message.show(); // Quit

    message = Message::Move { x: 30, y: 40 };
    message.show(); // Move { x: 30, y: 40 }

    message = Message::Write("Hello".to_string());
    message.show(); // Write("Hello")

    message = Message::ChangeColor(0, 255, 255);
    message.show(); // ChangeColor(0, 255, 255)
}

fn optional() {
    let mut num: Option<i32>;

    num = Some(100);
    println!("{:?}", num);

    num = None;
    println!("{:?}", num);
}

// match と if let でのパターンマッチングの書き方はたくさんあるので、以下リンクを参照
// https://cheats.rs/#pattern-matching
fn match_stmt() {
    let message = Message::Move { x: 30, y: 40 };
    match message {
        Message::Quit
            => println!("quit"), // 式を書く場所なのでセミコロンではなくカンマ
        Message::Move { x, y }
            => println!("move to ({}, {})", x, y),
        Message::Write(s)
            => println!("{}", s),
        Message::ChangeColor(r, g, b)
            => println!("color ({}, {}, {})", r, g, b),
    }
}
fn if_let() {
    let num = Some(60);
    let actual_num =
        if let Some(number) = num { // Swift と同じく if は式として使える
            number
        } else {
            0
        };
    println!("num.unwrap_or(0) = {}", actual_num);
}
