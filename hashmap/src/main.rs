// hashmap adalah object seperti di javascript kurang lebih seperti struct dengan key dan value
use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Lazio"), 10); // pertama adalah key kedua adalah value
    scores.insert(String::from("Vicardo"), 50); // ini adalah hal yang sama

    // jangan lupa bahwa ownership berlaku juga untuk value di hashmap
    // cara replace value hashmap
    scores.insert(String::from("Lazio"), 100);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }


}
