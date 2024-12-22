use std::collections::HashMap;

pub fn main08() {
    // ハッシュマップ
    learn_hash_map()
}

fn learn_hash_map() {
    // HashMap::new() で作成
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("{:?}", scores);

    // keys と values を zip して作成
    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![10, 15];
    let scores2: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("{:?}", scores2);
    // scores の型は HashMap<&String, &i32> (teams と init_scores の借用)
    // iter() ではなく into_iter() を使えばムーブされて HashMap<String, i32> になる

    // アクセス
    let score: Option<&i32> = scores.get(&String::from("Red"));
    println!("scores[\"Red\"] = {:?}", score);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }

    // 新規追加
    scores.insert(String::from("Green"), 40);
    // なければ追加
    scores.entry(String::from("Cyan")).or_insert(70);
    println!("{:?}", scores);

    // Blue, Yellow に +10点
    for team in "Blue Yellow".split_whitespace() {
        let team = team.to_string();
        let score = scores.entry(team).or_insert(0);
        *score += 10;
    }
    println!("{:?}", scores);
}
