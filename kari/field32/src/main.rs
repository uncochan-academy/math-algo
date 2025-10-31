mod field32;
use field32::*;

fn main() {
    let a = create_number(5);
    let b = create_number(7);
    let result = a * b;
    println!("5 * 7 = {}", result.value);

    let inv = inverse(a);
    println!("5の逆元 = {}", inv.value);
    println!("5 * {} = {}", inv.value, (a * inv).value);
}