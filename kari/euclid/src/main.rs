use std::io;

fn main() {
    let mut input = String::new();
    println!("最大公約数をつくります。スペース区切りで数字を入力してください。");
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut n: Vec<i32> = input//ベクトルにしている
        .trim()//前後の空白を削除
        .split_whitespace()//空白で分割
        .map(|s| s.parse().expect("parse error"))//各要素をi32に変換
        .collect();//vecに収集
    println!("{:?}", n);
    println!("次元は{}です。", n.len());

    let mut shelter : i32;
    let dim : usize = n.len();
    let mut counter : usize = 1;

    while counter < dim {
        while n[counter]%n[0] != 0 {
            n[0] = n[counter]%n[0];
            shelter = n[0];
            n[0] = n[counter];
            n[counter] = shelter;
        }
        counter += 1;
    }
    println!("最大公約数は{}です", n[0]);
}