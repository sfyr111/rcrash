fn main() {
    // for with range (exclusive end)
    for i in 0..5 {
        println!("i = {}", i);
    }
    println!("---");
    // for with range (inclusive end)
    for i in 1..=3 {
        println!("i = {}", i);
    }
    println!("---");
    // for over array
    let arr = [10, 20, 30];
    for val in arr.iter() {
        println!("val = {}", val);
    }
    println!("---");
    // for with enumerate (index + value)
    for (idx, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", idx, val);
    }
    println!("---");
    // for over chars in a string
    let s = "Rust";
    for c in s.chars() {
        println!("char = {}", c);
    }
}
