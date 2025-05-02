// ch09_05_hashmap.rs
// 9.5 HashMap in Rust
// Demonstrates the usage, features, and best practices for HashMap<K, V>.
//
// HashMap is a key-value store collection in Rust, implemented with hashing.
// Core scenarios for using HashMap:
// 1. When you want to associate keys with values for fast lookup, insertion, and removal.
// 2. When you need to count, group, or index data by keys.
// 3. When you want to efficiently check existence or update values by key.
// HashMap is the most common associative collection in Rust.
//
// Common HashMap methods:
// - insert, get, get_mut, remove, contains_key, entry, keys, values, iter, iter_mut
// - len, is_empty, clear, drain, extend, retain

use std::collections::HashMap;

fn main() {
    // 1. Creating a new HashMap and inserting key-value pairs
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("scores = {:?}", scores);

    // 2. Accessing values by key
    let team_name = "Blue";
    match scores.get(team_name) {
        Some(score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score for {}", team_name),
    }

    // 3. Iterating over key-value pairs (by reference)
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 3.1 Iterating over key-value pairs (by mutable reference)
    for (key, value) in &mut scores {
        println!("{}: {} (mutable)", key, value);
    }

    // 3.2 Iterating over keys only
    for key in scores.keys() {
        println!("key: {}", key);
    }

    // 3.3 Iterating over values only
    for value in scores.values() {
        println!("value: {}", value);
    }

    // 3.4 Consuming iteration (by value, moves the HashMap)
    let scores_clone = scores.clone();
    for (key, value) in scores_clone.into_iter() {
        println!("{}: {} (owned)", key, value);
    }

    // 3.5 Iterating with enumerate (index, (key, value))
    for (i, (key, value)) in scores.iter().enumerate() {
        println!("#{}: {} => {}", i, key, value);
    }

    // 4. Updating a value for a key
    scores.insert("Blue", 25);
    println!("After update: {:?}", scores);

    // 5. Only insert if key does not exist (entry API)
    scores.entry("Green").or_insert(30);
    println!("After entry or_insert: {:?}", scores);

    // 6. Modify value in place (entry API)
    scores.entry("Blue").and_modify(|v| *v += 10);
    println!("After entry and_modify: {:?}", scores);

    // 7. Remove a key
    scores.remove("Yellow");
    println!("After remove: {:?}", scores);

    // 8. Check existence
    println!("Contains 'Blue'? {}", scores.contains_key("Blue"));
    println!("Contains 'Yellow'? {}", scores.contains_key("Yellow"));

    // 9. Other useful methods
    println!("Length: {}, Is empty: {}", scores.len(), scores.is_empty());
    scores.clear();
    println!("After clear: {:?}", scores);
}
