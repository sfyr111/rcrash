// Demonstration of high-order functions in Rust

fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn main() {
    // Pass closure as argument
    let square = |x| x * x;
    let result = apply_twice(square, 2);
    println!("apply_twice(square, 2) = {}", result); // (2*2)*2*2 = 16

    // Use map, filter, fold (high-order methods on iterators)
    let numbers = vec![1, 2, 3, 4, 5];
    // Use pattern matching in map and filter for clarity and consistency
    let squares: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("squares = {:?}", squares);

    let even: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("even numbers = {:?}", even);

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    // Returning a function (closure)
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    let triple = make_multiplier(3);
    println!("triple(6) = {}", triple(6));
}
