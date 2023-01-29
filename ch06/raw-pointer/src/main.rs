fn main() {
    let a: i64 = 42;
    // 変数aへの参照を、i64への定数型生ポインタへとキャスト
    let a_ptr = &a as *const i64;

    // 変数aの値と、そのメモリアドレスを出力
    println!("a: {} (a_ptr: {:p})", a, a_ptr);
}
