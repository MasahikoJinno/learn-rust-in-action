## memo

イテレータの反復処理中にイテレータそのものを変更しようとするとコンパイルエラー

```bash
~/w/r/l/c/iter-invalidation ❯❯❯ cargo run                                                                                                       main ✚ ✱
   Compiling iter-invalidation v0.1.0 (/Users/mjinno/work/repo/learn-rust-in-action/ch01/iter-invalidation)
error[E0502]: cannot borrow `letters` as mutable because it is also borrowed as immutable
 --> src/main.rs:8:9
  |
5 |     for letter in letters.iter() {
  |                   --------------
  |                   |
  |                   immutable borrow occurs here
  |                   immutable borrow later used here
...
8 |         letters.push(letter.clone())
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `iter-invalidation` due to previous error

```