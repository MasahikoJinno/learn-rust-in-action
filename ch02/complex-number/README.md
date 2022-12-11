## memo

- Rustの標準ライブラリは軽量で他の言語なら利用できるような数値型を含んでいない
- クレートがあるのでそれを利用する ※ numクレートは特殊な数値型を扱える

### dependenciesに追加

Cargo.toml
```toml
[dependencies]
num = "0.4.0"
```

dependenciesに追加したら `cargo update` をする
```bash
$ cargo update
```
