# 9.5 HashMap in Rust

This chapter demonstrates the usage, features, and best practices for the hash map type `HashMap<K, V>`.

---

## When to Use HashMap?

HashMap is the most common associative collection in Rust. Use HashMap when:

1. **You want to associate keys with values for fast lookup, insertion, and removal**  
   - HashMap provides average O(1) lookup, insert, and remove by key.
2. **You need to count, group, or index data by keys**  
   - Useful for frequency counting, grouping, and indexing.
3. **You want to efficiently check existence or update values by key**  
   - The entry API allows conditional insert and in-place mutation.

---

## Example Code

```rust
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
```

## Commonly Used HashMap Methods

- `insert`, `get`, `get_mut`, `remove`, `contains_key`, `entry`, `keys`, `values`, `iter`, `iter_mut`
- `len`, `is_empty`, `clear`, `drain`, `extend`, `retain`

See the code example above for practical usage of these methods in business scenarios.

---

## Key Points

- `HashMap<K, V>` is a hash table-based, growable key-value store.
- Use cases:
    1. Fast key-based lookup and update
    2. Grouping, counting, or indexing by key
    3. Efficient existence check and conditional mutation
- Keys must implement `Eq` and `Hash` traits.
- The entry API enables flexible insert-or-update logic.
- Iteration order is not guaranteed.

---

Run the code:

```sh
cargo run --bin ch09_05_hashmap
