# field32

A Rust library implementing arithmetic operations over GF(2^32) (32-bit finite field).

**[日本語版 README はこちら / Japanese README](README_ja.md)**

## Overview

This library provides multiplication and multiplicative inverse calculation over the finite field

$$\text{GF}(2^{32})=\mathbb{F}_{2^{32}}$$

Finite field arithmetic is widely used in cryptography, error correction codes, and other applications. Unlike unsigned integers like u64 or u32, where the multiplicative inverse of

$$2$$

does not exist, in

$$\text{GF}(2^{32})$$

every non-zero element has a multiplicative inverse, making division possible.

## Algebraic Background

### Ring

A set

$$R$$

with elements

$$0,1\in R$$

and two binary operations "

$$+,\cdot$$

" satisfying the following axioms is called a ring:

1. For any

$$a,b,c\in R$$

we have

$$(a+b)+c=a+(b+c)$$

(associativity of addition)

2. For any

$$a\in R$$

we have

$$a+0=0+a=a$$

(additive identity)

3. For any

$$a\in R$$

there exists

$$b\in R$$

such that

$$a+b=b+a=0$$

(additive inverse)

4. For any

$$a,b\in R$$

we have

$$a+b=b+a$$

(commutativity of addition)

5. For any

$$a,b,c\in R$$

we have

$$(a\cdot b)\cdot c=a\cdot (b\cdot c)$$

(associativity of multiplication)

6. For any

$$a\in R$$

we have

$$a\cdot 1=1\cdot a=a$$

(multiplicative identity)

7. For any

$$a,b,c\in R$$

we have

$$a\cdot (b+c)=a\cdot b+a\cdot c,(a+b)\cdot c=a\cdot c+b\cdot c$$

(distributivity)

The "

$$\cdot$$

" symbol is often omitted.

### Commutative Ring

A ring

$$R$$

that satisfies the following axiom is called a commutative ring. Conversely, when there exists an element that does not satisfy this property, it is emphasized as a non-commutative ring.

8. For any

$$a,b\in R$$

we have

$$a\cdot b=b\cdot a$$

(commutativity of multiplication)

### Field

A commutative ring

$$R$$

that satisfies the following axiom is called a field. It is often denoted by

$$K$$

or

$$F$$

taken from the first letters in German or English.

9. For any non-zero

$$a\in R$$

there exists

$$b\in R$$

such that

$$a\cdot b=b\cdot a=1$$

(multiplicative inverse)

### Division Ring (Skew Field)

A non-commutative ring

$$R$$

that satisfies axiom 9 is called a division ring or skew field.

- **Examples of fields:**

  Complex numbers

$$\mathbb{C}$$

, real numbers

$$\mathbb{R}$$

,

$$\text{GF}(2^{32})$$

, etc.

- **Examples of division rings:**

  Hamilton's quaternions

$$\mathbb{H}$$

, general linear group

$$\text{GL}(K)$$

, etc.

- **Examples of commutative rings:**

  Integer ring

$$\mathbb{Z}$$

, unsigned integers u64, u32, etc.

- **Examples of non-commutative rings:**

  Matrix ring

$$M_n(K)$$

Finite division rings are always commutative:
>**Wedderburn's Little Theorem**
>
>Every finite division ring is a commutative field.

## About Finite Fields

GF(2^32) is a field with 2^32 elements. In this implementation:
- Elements are represented as 32-bit integers
- Addition is performed using XOR operation
- Multiplication is defined using modular reduction by an irreducible polynomial

### Irreducible Polynomial Used

```rust
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;
```

This corresponds to the irreducible polynomial

$$x^{32} + x^{18} + x^2 + x + 1$$

([reference](https://www.partow.net/programming/polynomials/index.html)).

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

## Detailed Explanation

### Inverse Calculation

The inverse is computed as follows:

First, we perform the Euclidean algorithm using general polynomial division (using the `poly_warizan` function), where addition is done with the XOR operation `^` and multiplication uses the `kakezan` function.

$$p\colon$$

Irreducible polynomial.

$$a\colon$$

The number whose inverse we want to find.

<div align="center">

$$p=q_1a+r_1$$

$$a=q_2r_1+r_2$$

$$r_1=q_3r_2+r_3$$

$$\vdots$$

$$r_{n-3}=q_{n-1}r_{n-2}+r_{n-1}$$

$$r_{n-2}=q_nr_{n-1}+r_n$$

</div>

Here

$$r_{n-1}=1,r_n=0$$

Writing this in matrix form:

<div align="center">
<table>
<tr>
  <td rowspan="2">⎛<br>⎝</td>
  <td>r<sub>i-2</sub></td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>q<sub>i</sub></td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>r<sub>i-1</sub></td>
  <td rowspan="2">⎞<br>⎠</td>
</tr>
<tr>
  <td>r<sub>i-1</sub></td>
  <td>1</td>
  <td>0</td>
  <td>r<sub>i</sub></td>
</tr>
</table>
</div>

Connecting these together:

<div align="center">
<table>
<tr>
  <td rowspan="2">⎛<br>⎝</td>
  <td>p</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>q<sub>1</sub></td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>q<sub>2</sub></td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;⋯&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>q<sub>n</sub></td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>r<sub>n-1</sub></td>
  <td rowspan="2">⎞<br>⎠</td>
</tr>
<tr>
  <td>a</td>
  <td>1</td>
  <td>0</td>
  <td>1</td>
  <td>0</td>
  <td>1</td>
  <td>0</td>
  <td>r<sub>n</sub></td>
</tr>
</table>
</div>

Multiplying from the left by the inverse matrix:

<div align="center">
<table>
<tr>
  <td rowspan="2">⎛<br>⎝</td>
  <td>q<sub>i</sub></td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2"><sup>-1</sup>&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
</tr>
<tr>
  <td>1</td>
  <td>0</td>
  <td>1</td>
  <td>q<sub>i</sub></td>
</tr>
</table>
</div>

We obtain:

<div align="center">
<table>
<tr>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;⋯&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>p</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>r<sub>n-1</sub></td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
</tr>
<tr>
  <td>1</td>
  <td>q<sub>n</sub></td>
  <td>1</td>
  <td>q<sub>n-1</sub></td>
  <td>1</td>
  <td>q<sub>1</sub></td>
  <td>a</td>
  <td>r<sub>n</sub></td>
  <td>0</td>
</tr>
</table>
</div>

In the code, we set:

<div align="center">
<table>
<tr>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;⋯&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>0</td>
  <td>1</td>
  <td rowspan="2">⎞<br>⎠</td>
  <td rowspan="2">&nbsp;=&nbsp;</td>
  <td rowspan="2">⎛<br>⎝</td>
  <td>v[0]</td>
  <td>v[1]</td>
  <td rowspan="2">⎞<br>⎠</td>
</tr>
<tr>
  <td>1</td>
  <td>q<sub>n</sub></td>
  <td>1</td>
  <td>q<sub>n-1</sub></td>
  <td>1</td>
  <td>q<sub>1</sub></td>
  <td>v[2]</td>
  <td>v[3]</td>
</tr>
</table>
</div>

Therefore:

$$pv[0]+av[1]=1$$

Considering

$$\text{mod}\:p$$

we have:

$$av[1]=1$$

Thus

$$v[1]$$

is the desired inverse.

## References

- [Irreducible Polynomials - Arash Partow](https://www.partow.net/programming/polynomials/index.html)
- General textbooks on finite field theory

## License

(Add as needed)
