mod field32;
use field32::*;
fn main() {
    let a = create_number(1635);
    let b = create_number(2432);
    let result = a * b;
    println!("1635*2472 = {}", result.value);

    let c: MyNumber = create_number(3);
    let result2 = inverse(c);
    println!("3の逆元 = {}", result2.value);

}