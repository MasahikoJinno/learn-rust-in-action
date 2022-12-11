## memo

このコードは複数のスレッドで同じ変数を書き込み、データ競合が発生する。
Rustはコンパイル時に検証をすることで、競合状態を予防してくれる。

error

```bash
~/w/r/l/c/race-condition ❯❯❯ cargo run                                                                                                          main ✚ ✱
   Compiling race-condition v0.1.0 (/Users/mjinno/work/repo/learn-rust-in-action/ch01/race-condition)
error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:6:19
  |
6 |     thread::spawn(|| { data = 500; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:6:5
  |
6 |     thread::spawn(|| { data = 500; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
6 |     thread::spawn(move || { data = 500; });
  |                   ++++

error[E0499]: cannot borrow `data` as mutable more than once at a time
 --> src/main.rs:7:19
  |
6 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             first mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
7 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- second borrow occurs due to use of `data` in closure
  |                   |
  |                   second mutable borrow occurs here

error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:7:19
  |
7 |     thread::spawn(|| { data = 1000; });
  |                   ^^   ---- `data` is borrowed here
  |                   |
  |                   may outlive borrowed value `data`
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:7:5
  |
7 |     thread::spawn(|| { data = 1000; });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
7 |     thread::spawn(move || { data = 1000; });
  |                   ++++

error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
 --> src/main.rs:8:20
  |
6 |     thread::spawn(|| { data = 500; });
  |     ---------------------------------
  |     |             |    |
  |     |             |    first borrow occurs due to use of `data` in closure
  |     |             mutable borrow occurs here
  |     argument requires that `data` is borrowed for `'static`
7 |     thread::spawn(|| { data = 1000; });
8 |     println!("{}", data);
  |                    ^^^^ immutable borrow occurs here
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0373, E0499, E0502.
For more information about an error, try `rustc --explain E0373`.
error: could not compile `race-condition` due to 4 previous errors
```