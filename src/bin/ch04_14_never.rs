// Demonstration of diverging (never type) functions in Rust

// A diverging function never returns, its return type is `!` (never type)
fn never_returns() -> ! {
    panic!("This function never returns!");
}

fn infinite_loop() -> ! {
    loop {
        // Infinite loop, never returns
    }
}

fn main() {
    println!("About to call a diverging function...");
    // Uncomment one of the lines below to see diverging behavior:
    // never_returns();
    // infinite_loop();
    println!("This line is after diverging functions (unreachable if called)");
}
