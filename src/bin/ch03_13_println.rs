fn main() {
    // Basic usage
    println!("Hello, world!");

    // Print variables
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);

    // Print with positional arguments
    println!("{1} is {0} years old.", age, name);

    // Print with named arguments
    println!("{person} scored {score}", person = name, score = 95);

    // Print debug info with {:?}
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);

    // Print pretty debug info with {:#?}
    println!("arr (pretty) = {:#?}", arr);

    // Print numbers in different bases
    let num = 255;
    println!("Decimal: {} Hex: {:x} Binary: {:b}", num, num, num);

    // Print with padding and alignment
    println!("|{:>5}|{:^5}|{:<5}|", 1, 2, 3); // right, center, left

    // Print floating point with precision
    let pi = 3.1415926;
    println!("pi = {:.2}", pi);
}
