fn main() {
    let fruit = vec!["apple", "banana", "cherry"];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "melon");
}
