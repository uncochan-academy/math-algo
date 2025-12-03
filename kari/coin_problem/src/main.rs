use rand::Rng;

// 1なら重い．ー１なら軽い．
fn random_assign() -> Vec<i32> {
    let mut arr = vec![0; 12];
    let mut rng = rand::thread_rng();

    // ランダムにインデックスを選択 (0-11)
    let index = rng.gen_range(0..12);

    // ランダムに1か-1を選択
    let value = if rng.gen_bool(0.5) { 1 } else { -1 };

    arr[index] = value;

    arr
}


// arrをみっつのグループに分ける関数．groupsの値が-1なら左．0なら中央．1なら右．
fn siwake(arr: &Vec<i32>, groups: &Vec<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut center = Vec::new();
    let mut right = Vec::new();

    for i in 0..arr.len() {
        match groups[i] {
            -1 => left.push(arr[i]),
            0 => center.push(arr[i]),
            1 => right.push(arr[i]),
            _ => {} // -1, 0, 1 以外は無視
        }
    }

    (left, center, right)
}

//右がしずむと１
fn tenbin(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let left_sum: i32 = left.iter().sum();
    let right_sum: i32 = right.iter().sum();
    let tenbin_result = right_sum - left_sum;
    match tenbin_result {
        x if x > 0 => 1,
        x if x < 0 => -1,
        0 => 0,
        _ => 736464738,
    }
}

// 1回目が右重い場合の処理
fn handle_right_heavy(x: &Vec<i32>) {
    let mut y = siwake(&x, &vec![-1,-1,1,0,1,1,0,0,-1,-1,1,0]);
    let mut z = tenbin(y.0, y.2);
    match z {
        1 => {
            y = siwake(&x, &vec![-1,1,0,0,0,0,0,0,0,0,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("１番目がー１"),
                -1 => println!("２番目がー１"),
                0 => println!("１１番目が１"),
                _ => println!("エラー"),
            }
        },
        -1 => {
            y = siwake(&x, &vec![0,0,0,0,0,0,0,0,-1,1,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("１０番目が１"),
                -1 => println!("９番目が１"),
                0 => println!("３番目がー１"),
                _ => println!("エラー"),
            }
        },
        0 => {
            y = siwake(&x, &vec![0,0,0,0,-1,0,0,0,0,0,0,1]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("１２番目が１"),
                0 => println!("４番目がー１"),
                _ => println!("エラー"),
            }
        },
        _ => println!("エラー"),
    }
}

// 1回目が左重い場合の処理
fn handle_left_heavy(x: &Vec<i32>) {
    let mut y = siwake(&x, &vec![-1,-1,1,0,1,1,0,0,-1,-1,1,0]);
    let mut z = tenbin(y.0, y.2);
    match z {
        1 => {
            y = siwake(&x, &vec![0,0,0,0,0,0,0,0,-1,1,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("９番目がー１"),
                -1 => println!("１０番目がー１"),
                0 => println!("３番目が１"),
                _ => println!("エラー"),
            }
        },
        -1 => {
            y = siwake(&x, &vec![-1,1,0,0,0,0,0,0,0,0,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("２番目が１"),
                -1 => println!("１番目が１"),
                0 => println!("１１番目がー１"),
                _ => println!("エラー"),
            }
        },
        0 => {
            y = siwake(&x, &vec![0,0,0,0,-1,0,0,0,0,0,0,1]);
            z = tenbin(y.0, y.2);
            match z {
                -1 => println!("１２番目がー１"),
                0 => println!("４番目が１"),
                _ => println!("エラー"),
            }
        },
        _ => println!("エラー"),
    }
}

// 1回目が釣り合った場合の処理
fn handle_balanced(x: &Vec<i32>) {
    let mut y = siwake(&x, &vec![1,0,0,0,-1,-1,1,0,0,0,0,0]);
    let mut z = tenbin(y.0, y.2);
    match z {
        1 => {
            y = siwake(&x, &vec![0,0,0,0,-1,1,0,0,0,0,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("５番目がー１"),
                -1 => println!("６番目がー１"),
                0 => println!("７番目が１"),
                _ => println!("エラー"),
            }
        }
        -1 => {
            y = siwake(&x, &vec![0,0,0,0,-1,1,0,0,0,0,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("６番目が１"),
                -1 => println!("５番目が１"),
                0 => println!("７番目がー１"),
                _ => println!("エラー"),
            }
        },
        0 => {
            y = siwake(&x, &vec![1,0,0,0,0,0,0,-1,0,0,0,0]);
            z = tenbin(y.0, y.2);
            match z {
                1 => println!("８番目がマイナス１"),
                -1 => println!("８番目が１"),
                _ => println!("エラー"),
            }
        },
        _ => println!("エラー"),
    }
}

// コイン問題を解く関数
fn solve_coin_problem(x: &Vec<i32>) {
    // 1回目の天秤
    let mut y = siwake(&x, &vec![-1,-1,-1,-1,0,0,0,0,1,1,1,1]);
    let mut z = tenbin(y.0, y.2);

    // 1回目の結果に応じて処理を分岐
    match z {
        1 => handle_right_heavy(&x),   // 右が重い
        -1 => handle_left_heavy(&x),   // 左が重い
        0 => handle_balanced(&x),      // 釣り合っている
        _ => println!("エラー"),
    }
}

// 24通りすべてのパターンを生成
fn generate_all_patterns() -> Vec<(Vec<i32>, &'static str)> {
    vec![
        (vec![-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "１番目がー１"),
        (vec![0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "２番目がー１"),
        (vec![0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "３番目がー１"),
        (vec![0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0], "４番目がー１"),
        (vec![0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0], "５番目がー１"),
        (vec![0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0], "６番目がー１"),
        (vec![0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0], "７番目がー１"),
        (vec![0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0], "８番目がマイナス１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0], "９番目がー１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0], "１０番目がー１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0], "１１番目がー１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1], "１２番目がー１"),
        (vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "１番目が１"),
        (vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "２番目が１"),
        (vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], "３番目が１"),
        (vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], "４番目が１"),
        (vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], "５番目が１"),
        (vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], "６番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], "７番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], "８番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], "９番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], "１０番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0], "１１番目が１"),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], "１２番目が１"),
    ]
}

fn main() {
    use std::io::{self, Write};

    println!("=== 24通りすべてのパターンをテスト ===\n");

    let patterns = generate_all_patterns();

    for (i, (pattern, expected)) in patterns.iter().enumerate() {
        print!("パターン{:2}: {:?} => 期待: {} => 結果: ", i + 1, pattern, expected);
        io::stdout().flush().unwrap();  // バッファをフラッシュ
        solve_coin_problem(pattern);
        println!();
    }

    println!("\n=== テスト完了 ===");
    println!("目視で結果が期待値と一致しているか確認してください");
}
