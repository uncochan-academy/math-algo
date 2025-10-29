use std::ops::Mul;

#[derive(Debug, Clone, Copy)]//なんかよくわからんもの
pub struct MyNumber {
    pub value: i32,
}

impl Mul for MyNumber {
    type Output = MyNumber;
    
    fn mul(self, other: MyNumber) -> MyNumber {
        MyNumber {
            value: (self.value as i64) ^ (other.value as i64),
        }
    }
}

pub fn create_number(value: i32) -> MyNumber {
    MyNumber { value }
}