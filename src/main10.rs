use std::fmt::Display;

pub fn main10() {
    // ジェネリックな関数の実装
    jeneric_function();

    // impl<T>
    jeneric_impl();

    // 型にトレイトを実装する (Swiftでは準拠させるという)
    impl_trait();
}

fn jeneric_function() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largetst: T = list[0]; // T: Copy なのでムーブは起きない
        for &number in list {
            if number > largetst {  // T: PartialOrd で比較
                largetst = number;
            }
        }
        largetst
    }

    let floats = [2.5, 3.14, 1., 9.8, 0.5];
    let max_float = largest(&floats);
    println!("largest: {}", max_float); // 9.8

    // トレイトの縛りを書く方法は 3 つある。
    trait _Example {
        // 方法1: 方法 2 と違って、例えば a: impl Copy, b: impl Copy で
        //       a, b の型が同じとは限らないことに注意
        fn method1(item: &(impl Copy + Display));
        // 方法2: <T> の中に書くパターン = トレイト境界構文 というシンタックスシュガー
        fn method2<T: Copy + Display>(item: &T);
        // 方法3: where で書くパターン
        fn method3<T>(item: &T)
        where
            T: Copy + Display;
    }
}

fn jeneric_impl() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    // ジェネリックな struct のメソッドを定義
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // T がトレイトをsatisfyする場合だけ使えるメソッドを定義
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(3, 7);
    pair.cmp_display(); // The largest number is y = 7
}

fn impl_trait() {
    trait Summary {
        // このようにトレイトがデフォルト実装を持つ場合もあるよ
        fn summarize(&self) -> String {
            String::from("デフォルト実装")
        }
    }

    struct Tweet {
        username: String,
        content: String,
    }

    // struct Tweet に trait Summary を実装する
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // Summary を実装する何かを返す。ただし戻り値の型は一つに定まる必要があるため、
    // この関数は Tweet 以外の型は返せない。複数の型を返したい場合は dyn Trait で検索
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("霽月"),
            content: String::from("Hello, world!"),
        }
    }

    let summ = returns_summarizable();
    summ.summarize();
}