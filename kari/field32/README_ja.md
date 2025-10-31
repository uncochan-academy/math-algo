# field32

GF(2^32)（32ビットの有限体）上での演算を実装したRustライブラリです。

## 概要

このライブラリは、32ビットの有限体 GF(2^32) 上での乗算と逆元計算を提供します。有限体の演算は、暗号技術やエラー訂正符号などで広く使用されています。

## 有限体について

有限体 GF(2^32) は、2^32個の要素を持つ体です。この実装では：
- 要素は32ビットの整数として表現されます
- 加算はXOR演算で行われます
- 乗算は既約多項式による剰余演算で定義されます

### 使用している既約多項式

```rust
const IRREDUCIBLE_POLYNOMIAL: u64 = 0x1_0040_0007;
```

これは `x^32 + x^18 + x^2 + x + 1` に対応する既約多項式です（[参考](https://www.partow.net/programming/polynomials/index.html)）。

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
