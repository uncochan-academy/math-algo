use core::panic;
use std::ops::Mul;
//https://www.partow.net/programming/polynomials/index.html
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;

#[derive(Debug, Clone, Copy)]//なんかよくわからんもの
pub struct MyNumber {
    pub value: u32,
}

impl Mul for MyNumber {
    type Output = MyNumber;
    
    fn mul(self, other: MyNumber) -> MyNumber {

        let mut result = kakezan(self.value as u64, other.value as u64);

        (_,result) = poly_warizan(result,IRREDUCIBLE_POLYNOMIAL);

        MyNumber {
            value: result as u32,
        }
    }
}

pub fn create_number(value: u32) -> MyNumber {
    MyNumber { value }
}

pub fn kakezan(a: u64, b: u64) -> u64 {
    //aかけるb.u32までしかちゃんと動かない
    let mut result: u64 =0;
    for i in 0..64 {
            if (b >> i) & 1 == 1 {
                result = result ^ (a << i);
            }
    }
    return result;
}

pub fn poly_warizan(a: u64, b: u64) -> (u64,u64) {
    //aわるbの商とあまり,bが０でないことは検証しないから注意
    //多項式bの最高次の次元dimを求める。
    let mut syou:u64 = 0;
    let mut amari:u64 = a;
    let dim = 63 - b.leading_zeros();
    for i in 0..(64 - dim) {
        if (amari >> (63 - i)) & 1 == 1 {
            amari = amari ^ (b << (63 - dim - i));
            syou = syou ^ (1 << (63 - dim - i));
        }
    }
    return (syou,amari);      
}
 
 // 逆元を求める関数
pub fn inverse(a: MyNumber) -> MyNumber {
    if a.value == 0 {
        panic!("0の逆元は存在しません");
    }

    let (p,a):(u64,u64) = (IRREDUCIBLE_POLYNOMIAL,a.value as u64);

    let (mut b,mut c):(u64,u64) = (p,a);

    let mut v: Vec<u64> = vec![1,0,0,1];

    let mut q:u64;

    let mut r:u64;

    while c != 0 {
        (q,r) = poly_warizan(b,c);
        (v[0],v[1],v[2],v[3]) =
            (v[2],v[3],
                v[0] ^ kakezan(q,v[2]),
                v[1] ^ kakezan(q,v[3])
            );
        b = c;
        c = r;
    }

    let result =poly_warizan(v[1],IRREDUCIBLE_POLYNOMIAL).1;
    return MyNumber { value: result as u32 };




}