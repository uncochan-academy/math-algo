mod field32;
use field32::*;

fn main() {
    // 32ビット以下の最大のフィボナッチ数を計算
    let mut fib_numbers = Vec::new();
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    while b <= u32::MAX as u64 {
        fib_numbers.push(b as u32);
        let next = a + b;
        a = b;
        b = next;
    }

    println!("32ビット以下のフィボナッチ数: {} 個", fib_numbers.len());
    println!("最大のフィボナッチ数: {}", fib_numbers.last().unwrap());

    // 最大のフィボナッチ数でテスト
    let max_fib = *fib_numbers.last().unwrap();
    println!("\n=== {}でテスト（最大のフィボナッチ数） ===", max_fib);

    let num = create_number(max_fib);
    let inv = inverse(num);
    let result = num * inv;

    println!("inverse({}) = {}", max_fib, inv.value);
    println!("{} * {} = {}", max_fib, inv.value, result.value);

    if result.value == 1 {
        println!("✓ 成功！");
    } else {
        println!("✗ 失敗！");
    }
}