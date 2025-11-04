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

        let mut result = kakezan(self.value as u64, other.value as u64).unwrap();

        (_,result) = poly_warizan(result,IRREDUCIBLE_POLYNOMIAL).unwrap();

        MyNumber {
            value: result as u32,
        }
    }
}

pub fn create_number(value: u32) -> MyNumber {
    MyNumber { value }
}

pub fn kakezan(a: u64, b: u64) -> Result<u64,String> {
    if ((a >> 32)|(b >> 32)) != 0 {
        return Err("３２ビットまでだぞ".to_string());
    }else{
        let mut result: u64 =0;
        for i in 0..64 {
                if (b >> i) & 1 == 1 {
                    result = result ^ (a << i);
                }
        }
        return Ok(result);
    }
}

pub fn poly_warizan(a: u64, b: u64) -> Result<(u64,u64),String> {
    if b == 0 {
        return Err("０で割ってるぞ".to_string());
    }else{
        //aわるbの商とあまり
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
        return Ok((syou,amari));
    }      
}
 
 // 逆元を求める関数
pub fn inverse(a: MyNumber) -> Result<MyNumber, String> {
    if a.value == 0 {
        return Err("０の逆元は存在しないぞ".to_string());
    }else{

        let (p,a):(u64,u64) = (IRREDUCIBLE_POLYNOMIAL,a.value as u64);

        let (mut b,mut c):(u64,u64) = (p,a);

        let mut v: Vec<u64> = vec![1,0,0,1];

        let mut q:u64;

        let mut r:u64;

        while c != 0 {
            (q,r) = poly_warizan(b,c).unwrap();
            (v[0],v[1],v[2],v[3]) =
                (v[2],v[3],
                    poly_warizan(v[0] ^ kakezan(q,v[2]).unwrap(),IRREDUCIBLE_POLYNOMIAL).unwrap().1,
                    poly_warizan(v[1] ^ kakezan(q,v[3]).unwrap(),IRREDUCIBLE_POLYNOMIAL).unwrap().1
                );
            b = c;
            c = r;
        }

        let result =poly_warizan(v[1],IRREDUCIBLE_POLYNOMIAL).unwrap().1;
        return Ok( MyNumber { value: result as u32 });

    }


}


//見やすく多項式にする関数

pub fn takousiki(a: MyNumber) -> String {
    if a.value == 0 {
        return "0".to_string();
    }
    let mut poly: Vec<String> = Vec::new();

    for i in (0..32).rev() {
        if ((a.value >> i) & 1) == 1 {
            match i {
                0 => poly.push("1".to_string()),
                1 => poly.push("x".to_string()),
                _ => poly.push(format!("x{}",to_superscript(i))),
            }
        }
    }
    poly.join(" + ")
}

fn to_superscript(n: u32) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => c,
        })
        .collect()
}