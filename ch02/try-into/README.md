## memo

- Rustでは型が違うと比較ができない
- `b.try_into()` が返すのはi32型の値の数値をResultでラップしたもの
- `unwrap()` メソッドはResultに入れられている成功の値を取り出す