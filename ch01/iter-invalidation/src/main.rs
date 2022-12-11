fn main() {
    // 可変（mutable）なベクター変数を作成
    let mut letters = vec!["a", "b", "c"];

    for letter in letters.iter() {
        println!("{}", letter);
        // 文字をコピーしてベクターの末尾に追加
        letters.push(letter.clone())
    }
}
