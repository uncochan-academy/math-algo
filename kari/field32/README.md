# field32

A Rust library implementing arithmetic operations over GF(2^32) (32-bit finite field).

**[日本語版 README はこちら / Japanese README](README_ja.md)**

## Overview

This library provides multiplication and multiplicative inverse calculation over the finite field GF(2^32). Finite field arithmetic is widely used in cryptography, error correction codes, and other applications.

## About Finite Fields

GF(2^32) is a field with 2^32 elements. In this implementation:
- Elements are represented as 32-bit integers
- Addition is performed using XOR operation
- Multiplication is defined using modular reduction by an irreducible polynomial

### Irreducible Polynomial

```rust
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;
```

This corresponds to the irreducible polynomial `x^32 + x^18 + x^2 + x + 1` ([reference](https://www.partow.net/programming/polynomials/index.html)).

## Main Features

### 1. MyNumber Type

A type representing a 32-bit finite field element.

```rust
pub struct MyNumber {
    pub value: u32,
}
```

### 2. Multiplication Operation

Implements the `Mul` trait, allowing multiplication using the `*` operator.

```rust
let a = create_number(1635);
let b = create_number(2432);
let result = a * b;
```

**Implementation flow:**
1. Multiply two 32-bit values as polynomials (`kakezan` function)
2. Calculate the remainder when divided by the irreducible polynomial (`poly_warizan` function)
3. Return the result as a 32-bit element

### 3. Multiplicative Inverse

The `inverse` function calculates the multiplicative inverse of a given element.

```rust
let c = create_number(5);
let inv = inverse(c);
// c * inv = 1 (in GF(2^32))
assert_eq!((c * inv).value, 1);
```

**Algorithm:**
Uses the Extended Euclidean Algorithm to compute the inverse.

1. Initialize with irreducible polynomial `p` and input value `a`
2. Compute `gcd(p, a)` using Euclidean algorithm while updating coefficient matrix
3. Extract the inverse from the final coefficient matrix

Matrix `v = [[v[0], v[1]], [v[2], v[3]]]` starts as identity matrix and updates at each step:
- Calculate quotient `q`
- Update matrix: new row = old row ^ (q * older row)

## Helper Functions

### kakezan(a: u64, b: u64) -> u64

Performs polynomial multiplication over GF(2^n).
- Not regular multiplication, but XOR-based carry-less multiplication
- Applies shift and XOR for each bit

### poly_warizan(a: u64, b: u64) -> (u64, u64)

Performs polynomial division and returns quotient and remainder.
- Return value: `(quotient, remainder)`
- Finds the highest bit position and successively subtracts using XOR

## Usage Example

```rust
use field32::*;

fn main() {
    // Multiplication example
    let a = create_number(1635);
    let b = create_number(2432);
    let result = a * b;
    println!("1635 * 2432 = {}", result.value);

    // Inverse example
    let c = create_number(3);
    let inv = inverse(c);
    println!("inverse of 3 = {}", inv.value);

    // Verification: c * inv = 1
    let one = c * inv;
    println!("3 * inverse(3) = {}", one.value); // Should be 1
}
```

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

## Technical Details

### XOR-based Operations

In GF(2^n), addition is equivalent to XOR operation:
- `a + b = a ^ b` (exclusive OR)
- No carry occurs

### Polynomial Representation

32-bit integers are interpreted as polynomials:
- `0x00000003` = `x + 1`
- `0x00000007` = `x^2 + x + 1`
- Most significant bit represents the highest degree coefficient

### Role of Irreducible Polynomial

The irreducible polynomial is used to keep multiplication results within 32 bits.
- The final result is the remainder after dividing by the irreducible polynomial
- This preserves the field properties of GF(2^32)

## Notes

- The `kakezan` function can handle values up to 64 bits, but operates correctly only with 32-bit inputs
- The `poly_warizan` function assumes the divisor is non-zero (no validation)
- The `inverse` function panics when given 0 (since the multiplicative inverse of 0 does not exist)

## References

- [Irreducible Polynomials - Arash Partow](https://www.partow.net/programming/polynomials/index.html)
- General textbooks on finite field theory

## License

(Add as needed)
