fn main() {
    // Single-line comment: use // for normal comments
    let x = 5; // This is an inline comment

    /*
        Multi-line comment: use /* ... */
        Good for temporarily disabling code or writing longer notes.
    */
    let y = 10;

    /// Documentation comment: use triple slashes (///)
    /// This function adds two numbers.
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sum = add(x, y);
    println!("sum = {}", sum);

    println!("x = {}, y = {}", x, y); // Output variables
}
