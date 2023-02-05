use bincode::serialize as to_bincode;
use serde_cbor::ser::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    lat: f32,
    lon: f32,
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 100000,
        lat: 4.5,
        lon: 8.5,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json: \n{}\n", as_json);
    println!("cbor: \n{:?}\n", as_cbor);
    println!("bincode: \n{:?}\n", as_bincode);

    println!("json (as UTF-8): \n{}\n", String::from_utf8_lossy(&as_json.as_bytes()));
    println!("cbor (as UTF-8): \n{:?}\n", String::from_utf8_lossy(&as_cbor));
    println!("bincode (as UTF-8): \n{:?}\n", String::from_utf8_lossy(&as_bincode));
}
