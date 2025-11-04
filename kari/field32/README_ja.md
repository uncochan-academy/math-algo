# field32

$\text{GF}(2^{32})=\mathbb{F}_{2^{32}}$（32ビットの有限体）上での演算を実装したRustライブラリです。

## 概要

このライブラリは、32ビットの有限体 
$\text{GF}(2^{32})$
上での乗算と逆元計算を提供します。有限体の演算は、暗号技術やエラー訂正符号などで広く使用されています。符号なし整数u64やu32には
$2$
などの逆数が存在しません。
$\text{GF}(2^{32})$
ではすべての逆数が存在し、除算ができます。

## 代数的な知識

### 環

集合
$R$
と
$0,1\in R $
なるものと二つの二項演算"
$+,\cdot$
"が以下の公理を満たすとき、環という。

１．任意の
$a,b,c\in R$
に対して
$(a+b)+c=a+(b+c)$
である

２．任意の
$a\in R$
に対して
$a+0=0+a=a$
である

３．任意の
$a\in R$
に対してある
$b\in R$
が存在して
$a+b=b+a=0$
となる

４．任意の
$a,b\in R$
に対して
$a+b=b+a$
である

５．任意の
$a,b,c\in R$
に対して
$(a\cdot b)\cdot c=a\cdot (b\cdot c)$
である

６．任意の
$a\in R$
に対して
$a\cdot 1=1\cdot a=a$
である

７．任意の
$a,b,c\in R$
に対して
$a\cdot (b+c)=a\cdot b+a\cdot c,(a+b)\cdot c=a\cdot c+b\cdot c$
である

基本的には"
$\cdot$
"は省略されることが多い。

### 可換環

環
$R$
が次の公理を満たすとき可換環という。逆に以下の性質を満たさない元が存在するときに、強調して非可換環とよばれることがある。

８．任意の
$a,b\in R$
に対して
$a\cdot b=b\cdot a$
である

### 体

可換環
$R$
が以下の公理を満たすとき体とよばれる。ドイツ語や英語の頭文字をとって
$K$
や
$F$
と表されることがある。

９．任意の
$a\in R$
に対してある元
$b\in R$
が存在して
$a\cdot b=b\cdot a=1$
となる

### 斜体

非可換環
$R$
が公理９を満たすとき
$R$
を斜体という。

- 体の例

複素数体
$\mathbb{C}$
，実数体
$\mathbb{R}$
，
$\text{GF}(2^{32})$
など


- 斜体の例

ハミルトンの四元数
$\mathbb{H}$
，一般線型群
$\text{GL}(K)$
など


- 可換環の例

整数環
$\mathbb{Z}$
，符号なし整数u64,u32など


- 非可換環の例

行列環
$M_n(K)$

元の個数が有限の斜体は必ず積が可換になります。
>**Wedderburnの小定理**
>
>有限な斜体は可換な体である。


### 使用している既約多項式

```rust
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;
```

これは 
$x^{32} + x^{18} + x^2 + x + 1$
 に対応する既約多項式です（[参考](https://www.partow.net/programming/polynomials/index.html)）。

## 主要な機能

### 1. MyNumber 型

32ビットの有限体要素を表す型です。

```rust
pub struct MyNumber {
    pub value: u32,
}
```

### 2. 乗算演算

`Mul` トレイトを実装しており、`*` 演算子で乗算ができます。

```rust
let a = create_number(1635);
let b = create_number(2432);
let result = a * b;
```

**実装の流れ：**
1. 2つの32ビット値を多項式として掛け算（`kakezan`関数）
2. 既約多項式で割った余りを計算（`poly_warizan`関数）
3. 結果を32ビットの要素として返す

### 3. 逆元計算

`inverse` 関数で、与えられた要素の乗法逆元を計算します。

```rust
let c = create_number(5);
let inv = inverse(c);
// c * inv = 1 (GF(2^32)上で)
assert_eq!((c * inv).value, 1);
```

**実装アルゴリズム：**
拡張ユークリッドの互除法を使用して逆元を計算しています。

1. 既約多項式 `p` と入力値 `a` を初期値とする
2. ユークリッドの互除法で `gcd(p, a)` を計算しながら、係数行列を更新
3. 最終的な係数行列から逆元を抽出

行列 `v = [[v[0], v[1]], [v[2], v[3]]]` を単位行列から開始し、各ステップで以下の更新を行います：
- 商 `q` を計算
- 行列を更新：新しい行 = 古い行 ^ (q * さらに古い行)

## 補助関数

### kakezan(a: u64, b: u64) -> u64

GF(2^n)上での多項式の乗算を行います。
- 通常の乗算ではなく、XORベースのキャリーなし乗算
- 各ビットに対してシフトとXORを適用

### poly_warizan(a: u64, b: u64) -> (u64, u64)

多項式の割り算を行い、商と余りを返します。
- 戻り値：`(商, 余り)`
- 最高次のビット位置を見つけて、順次XORで引いていく

## 使用例

```rust
use field32::*;

fn main() {
    // 乗算の例
    let a = create_number(1635);
    let b = create_number(2432);
    let result = a * b;
    println!("1635 * 2432 = {}", result.value);

    // 逆元の例
    let c = create_number(3);
    let inv = inverse(c);
    println!("3の逆元 = {}", inv.value);

    // 検証: c * inv = 1
    let one = c * inv;
    println!("3 * inverse(3) = {}", one.value); // 1になるはず
}
```

## ビルド

```bash
cargo build --release
```

## 実行

```bash
cargo run
```

## 技術的な詳細

### XORベースの演算

GF(2^n)では、加算はXOR演算と等価です：
- `a + b = a ^ b`（排他的論理和）
- キャリーが発生しない

### 多項式表現

32ビット整数は、次のような多項式として解釈されます：
- `0x00000003` = `x + 1`
- `0x00000007` = `x^2 + x + 1`
- 最上位ビットが最高次の係数

### 既約多項式の役割

既約多項式は、多項式の乗算結果を32ビット以内に収めるために使用されます。
- 乗算結果を既約多項式で割った余りが最終結果
- これにより、GF(2^32)の体の性質が保たれる

## 注意事項

- `kakezan`関数は64ビットまでの値を扱えますが、正確に動作するのは32ビット入力の場合です
- `poly_warizan`関数は除数が0でないことを前提としています（検証なし）
- `inverse`関数で0を渡すとパニックします（0の逆元は存在しないため）

## 参考文献

- [Irreducible Polynomials - Arash Partow](https://www.partow.net/programming/polynomials/index.html)
- 有限体論の一般的な教科書

## ライセンス

（必要に応じて追加してください）

## 他
- 逆元については以下のように求めています。

まずは一般的な多項式の割り算(このときはpoly_warizan)でユークリッドの互除法をします。ただし足し算はビット演算の^で、掛け算はkakezan関数を使っています。

$p\colon$既約多項式.

$a\colon$逆元を求めたい数.

$$
\begin{align*}
p&=q_1a+r_1\\
a&=q_2r_1+r_2\\
r_1&=q_3r_2+r_3\\
&\:\:\vdots\\
r_{n-3}&=q_{n-1}r_{n-2}+r_{n-1}\\
r_{n-2}&=q_nr_{n-1}+r_n\\
\end{align*}
$$
ここで
$r_{n-1}=1,r_n=0$
となる。行列で書くと

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

となっている。これをつなげて書くと

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

となる。ここに左から逆行列；

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

をかけていきます。すると

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

となります。コード上では

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

とおいています。よって

$$pv[0]+av[1]=1$$

となり$\text{mod}\:p$でかんがえれば

$$av[1]=1$$

となり$v[1]$が求める逆元となります。




