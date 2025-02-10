use std::collections::HashMap;

fn main() {
    test();
}

fn test() {
    let mut my_map = HashMap::new();
    my_map.insert("Hello", "World");

    println!("{}", my_map["Hello"]);
}