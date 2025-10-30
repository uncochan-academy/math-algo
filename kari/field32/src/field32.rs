use std::ops::Mul;
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;

#[derive(Debug, Clone, Copy)]//なんかよくわからんもの
pub struct MyNumber {
    pub value: u32,
}

impl Mul for MyNumber {
    type Output = MyNumber;
    
    fn mul(self, other: MyNumber) -> MyNumber {
        let mut result: u64 =0;

        for i in 0..32 {
            if (other.value >> i) & 1 == 1 {
                result = result ^ ((self.value as u64) << i);
            }
        }

        for i in 0..32 {
            if (result >> (63 - i)) & 1 == 1 {
                result = result ^ (IRREDUCIBLE_POLYNOMIAL << (31 - i));
            }
        }


        MyNumber {
            value: result as u32,
        }
    }
}

pub fn create_number(value: u32) -> MyNumber {
    MyNumber { value }
}