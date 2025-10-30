mod field32;
use field32::*;
fn main() {
    let a = create_number(3);
    let b = create_number(3);
    let result = a * b;
    println!("3 * 3 = {}", result.value);
}