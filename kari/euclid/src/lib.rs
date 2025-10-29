/// Compute the greatest common divisor of a non-empty slice of i32 values.
///
/// Returns 0 for an empty slice. All numbers are treated by absolute value.
pub fn gcd(x : Vec<i32>) -> i32 {
    if x.contains(&0) {
        println!("0が含まれています");
        return 0;
    }
    let mut n = x;

    let mut shelter : i32;
    let dim : usize = n.len();
    let mut counter : usize = 1;

    while counter < dim {
        while n[counter]%n[counter - 1] != 0 {
            n[counter] = n[counter]%n[counter - 1];
            shelter = n[counter - 1];
            n[counter - 1] = n[counter];
            n[counter] = shelter;
        }
        n[counter] = n[counter - 1];
        counter += 1;
    }
    return n[dim - 2];
}