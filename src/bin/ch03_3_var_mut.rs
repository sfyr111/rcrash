fn main() {
    // Constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);

    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This line would cause an error because x is immutable

    // Mutable variable
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20; // This is allowed because y is mutable
    println!("The value of y after mutation is: {}", y);

    // Variable shadowing
    let x = x + 1;
    println!("The value of x after first shadowing (x + 1): {}", x);
    let x = x * 2;
    println!("The value of x after second shadowing (x * 2): {}", x);
    {
        let x = x - 3;
        println!("The value of x in the inner scope after shadowing (x - 3): {}", x);
    }
    println!("The value of x in the outer scope after all shadowing: {}", x);
}
