use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 20);

    score.entry(String::from("Black")).or_insert(5);
    score.entry(String::from("Blue")).or_insert(70);

    for (key, value) in score {
        println!("{key} : {value}");
    };
}
