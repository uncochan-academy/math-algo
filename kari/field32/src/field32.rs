use std::ops::Mul;
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;

#[derive(Debug, Clone, Copy)]//なんかよくわからんもの
pub struct MyNumber {
    pub value: u32,
}

impl Mul for MyNumber {
    type Output = MyNumber;
    
    fn mul(self, other: MyNumber) -> MyNumber {

        let mut result = kakezan(self.value, other.value);

        result = poly_warizan(result,IRREDUCIBLE_POLYNOMIAL);

        MyNumber {
            value: result as u32,
        }
    }
}

pub fn create_number(value: u32) -> MyNumber {
    MyNumber { value }
}

pub fn kakezan(a: u32, b: u32) -> u64 {
    //aかけるb
    let mut result: u64 =0;
    for i in 0..32 {
            if (b >> i) & 1 == 1 {
                result = result ^ ((a as u64) << i);
            }
    }
    return result;
}

pub fn poly_warizan(mut a: u64, b: u64) -> u64 {
    //aわるbのあまり,bが０でないことは検証しないから注意
    //多項式bの最高次の次元dimを求める。
    let dim = 63 - b.leading_zeros();
    for i in 0..(64 - dim) {
        if (a >> (63 - i)) & 1 == 1 {
            a = a ^ (b << (63 - dim - i));
        }
    }
    return a;
      
}