#[derive(Debug)]
struct Rectangle {  // 構造体
    width: u32,     // フィールド
    height: u32,
}

impl Rectangle {
    // static メソッドのようなもの
    fn square(width: u32) -> Self {
        Self {
            width,  // "width: width" のように同じ名前の変数の場合は省略して書ける
            height: width
        }
    }

    // メソッドのようなもの Pythonのように最初に self がつく
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 自分に対して変更を加えるメソッドのようなもの
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

}

pub fn main04() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    let square = Rectangle::square(60);
    let rect2 = Rectangle {
        height: 120,
        ..square
    };

    println!("area of rect: {}", rect.area());  // 1500

    // #[derive(Debug)] でデバッグ出力が可能に
    println!("rect: {:?}", &rect);      // Rectangle { width: 30, height: 50 }
    println!("square: {:?}", &square);  // Rectangle { width: 60, height: 60 }
    println!("rect2: {:?}", &rect2);    // Rectangle { width: 60, height: 120 }
    // 改行付きで出力              // Rectangle {
    println!("{:#?}", &rect);   //     width: 30,
                                //     height: 50,
                                // }

    rect.set_width(40);
    println!("rect: {:?}", &rect);      // Rectangle { width: 40, height: 50 }

    // 【タプル構造体】 フィールドに名前のついていない構造体
    struct Color(i32, i32, i32);
    let cyan = Color(0, 255, 255);
    println!("cyan = #rgb({}, {}, {})", cyan.0, cyan.1, cyan.2);
}
