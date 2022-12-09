fn greet_world() {
    let japan = "こんにちは世界";
    let italy = "Ciao mondo";
    let france = "Bonjour le monde";
    let germany = "Hallo Welt";
    let england = "Hello, world";
    let regions = [japan, italy, france, germany, england];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
