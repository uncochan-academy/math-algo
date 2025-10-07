pub fn gcd(x : Vec<i32>) {
    let mut n = x;

    let mut shelter : i32;
    let dim : usize = n.len();
    let mut counter : usize = 1;

    while counter < dim {
        while n[counter]%n[0] != 0 {
            n[0] = n[counter]%n[0];
            shelter = n[0];
            n[0] = n[counter];
            n[counter] = shelter;
        }
        counter += 1;
    }
    println!("最大公約数は{}です", n[0]);
}