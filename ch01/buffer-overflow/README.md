## memo
- 無効なindexを指定するとpanicを起こす
- `assert_eq!` -> テスト用のマクロ、2つの引数が正しいか判定


```bash
~/w/r/l/c/buffer-overflow ❯❯❯ cargo run                                                                                                         main ✚ ✱
   Compiling buffer-overflow v0.1.0 (/Users/mjinno/work/repo/learn-rust-in-action/ch01/buffer-overflow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/buffer-overflow`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src/main.rs:4:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```