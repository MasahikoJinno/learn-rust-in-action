## memo
- `eprintln!` -> 標準エラー出力
- `cfg!(debug_assertions)` -> デバッグビルドかどうか判定
  - `cargo run` -> デバッグビルド
  - `cargo build` -> デバッグビルド
  - `cargo build --release` -> リリースビルド
- `Ok(length)` -> Result型で成功になった場合、lengthに値を入れる ※ このサンプルコードだと`parse()`が成功したら
  - `Err(E)` -> エラーを受け取るときはこうする（このコードだと浮動小数点にできない場合）
