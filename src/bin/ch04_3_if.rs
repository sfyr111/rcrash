fn main() {
    // Basic if/else
    let n = 7;
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }

    // if as an expression
    let is_even = if n % 2 == 0 { true } else { false };
    println!("{} is even? {}", n, is_even);

    // Nested if/else
    let grade = 85;
    let level = if grade >= 90 {
        "A"
    } else if grade >= 80 {
        "B"
    } else if grade >= 70 {
        "C"
    } else {
        "D or lower"
    };
    println!("grade {} -> level {}", grade, level);

    // if let (pattern matching shortcut)
    let opt = Some(10);
    if let Some(val) = opt {
        println!("if let: value is {}", val);
    } else {
        println!("if let: None");
    }
}
