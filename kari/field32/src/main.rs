mod field32;
use field32::*;
use std::io::{self, Write};

fn main() {
    println!("数字を入力してください (0以外の32ビット整数):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(value) => {
            let num = create_number(value);

            println!("{}",takousiki(num));

            match inverse(num) {
                Ok(inv) => {
                    let result = num * inv;

                    println!("\n結果:");
                    println!("inverse({}) = {} = {}", value, inv.value, takousiki(inv));
                    println!("{} * {} = {}", value, inv.value, result.value);

                    if result.value == 1 {
                        println!("✓ 検証成功！逆元が正しく計算されました。");
                    } else {
                        println!("✗ 検証失敗！");
                    }
                }
                Err(e) => {
                    println!("エラー: {}", e);
                }
            }
        }
        Err(_) => {
            println!("エラー: 有効な32ビット整数を入力してください。");
        }
    }
}