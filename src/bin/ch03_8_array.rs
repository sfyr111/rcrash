fn main() {
    // Creating an array with explicit type and length
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

    // Accessing array elements by index
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[4]);

    // Getting array length
    println!("Array length: {}", arr.len());

    // Iterating over array elements
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, val);
    }

    // Initializing an array with the same value
    let zeros = [0; 8];
    println!("Array of zeros: {:?}", zeros);

    // Slicing an array
    let slice = &arr[1..4];
    println!("Slice of arr[1..4]: {:?}", slice);

    // Using a computed value as an index
    let idx = 2 + 1; // Example: computed index
    println!("Element at computed index {}: {}", idx, arr[idx]);

    // Mutating array elements using a computed index
    let update_idx = arr.len() - 1;
    arr[update_idx] = 42;
    println!("After mutation, arr: {:?}", arr);

    // Using a string as an index (parse string to usize), step by step
    let str_index = "2";
    let parsed = str_index.parse::<usize>();
    if parsed.is_ok() {
        let i = parsed.unwrap();
        if i < arr.len() {
            println!("arr[{}] (from string index) = {}", str_index, arr[i]);
        } else {
            println!("Index {} out of bounds!", i);
        }
    } else {
        println!("Failed to parse index '{}': {}", str_index, parsed.unwrap_err());
    }

    // String indexing and slicing
    let s = String::from("hello, ");
    println!("String: {}", s);
    // String slicing (must be on valid char boundaries)
    let sub = &s[0..5]; // 'hello'
    println!("First 5 bytes as slice: {}", sub);
    // Attempting to index a string directly is not allowed:
    // let ch = s[1]; // This line would not compile
    // Instead, use chars().nth()
    let nth = s.chars().nth(1);
    if nth.is_some() {
        println!("Second character: {}", nth.unwrap());
    } else {
        println!("No second character");
    }
}
