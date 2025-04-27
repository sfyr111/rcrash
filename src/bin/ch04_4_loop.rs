fn main() {
    // Basic loop: infinite unless break
    let mut count = 0;
    loop {
        count += 1;
        println!("count = {}", count);
        if count == 3 {
            println!("Break at count = 3");
            break;
        }
    }

    // loop as an expression (returns value with break)
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 5 {
            break n * 10; // returns 50
        }
    };
    println!("Result from loop = {}", result);

    // Nested loop with labels
    let mut outer = 0;
    'outer_loop: loop {
        let mut inner = 0;
        loop {
            println!("outer={}, inner={}", outer, inner);
            if inner == 2 { break; }
            if outer == 1 { break 'outer_loop; }
            inner += 1;
        }
        outer += 1;
    }
    println!("Exited nested loop");
}
