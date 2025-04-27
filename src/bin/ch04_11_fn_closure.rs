fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}

fn main() {
    // Regular function
    let sum = add(2, 3);
    println!("add(2, 3) = {}", sum);

    // Closure: basic usage
    let closure_add = |a: i32, b: i32| a + b;
    println!("closure_add(2, 3) = {}", closure_add(2, 3));

    // Closure: type inference
    let multiply = |x, y| x * y;
    println!("multiply(4, 5) = {}", multiply(4, 5));

    // Closure capturing environment
    let offset = 10;
    let add_offset = |x| x + offset;
    println!("add_offset(5) = {}", add_offset(5));

    // Passing closure to function
    let result = apply(|x, y| x - y, 8, 3);
    println!("apply(|x, y| x - y, 8, 3) = {}", result);

    // Returning closure (requires Box)
    fn make_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |b| a + b)
    }
    let adder = make_adder(100);
    println!("adder(23) = {}", adder(23));
}
