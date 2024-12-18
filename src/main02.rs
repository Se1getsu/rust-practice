pub fn main02() {

    cond_stmt();

    loop_stmt();
}

fn cond_stmt() {
    let x = 5;
    if x != 0 {     // != 0 は必要
        println!("x not equals to 0");
    } else if x == 0 {
        println!("x equals to 0");
    } else {
        println!("unreachable block");
    }

    let n = if x > 0 { ">0" } else { "<=0" };
    println!("{}", n); // >0
}

fn loop_stmt() {
    loop {
        println!("無限ループ");
        break;
    }
    'label: loop {
        break 'label // ラベル付き break (n重ループで抜けるループを指定する用)
    }

    let mut i = 0;
    while i < 10 { i += 1; } // i = 10

    for i in 0..5 {
        println!("i = {}", i) // i = 0,1,2,3,4
    }
    for j in (0..5).rev() {
        println!("j = {}", j) // i = 4,3,2,1,0
    }
}
