fn return_tuple() -> (i32, f64, &'static str) {
    (7, 3.14, "tuple from function")
}

fn main() {
    // Creating a tuple with different types
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    println!("Tuple: {:?}", tup);

    // Accessing tuple elements by index
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("First: {}, Second: {}, Third: {}", first, second, third);

    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("Destructured: x = {}, y = {}, z = {}", x, y, z);

    // Nested tuple
    let nested = ((1, 2), (3.0, false));
    println!("Nested tuple: {:?}", nested);

    // Single-element tuple vs. value
    let single = (42,);
    let not_a_tuple = 42;
    println!("Single-element tuple: {:?}", single);
    println!("Not a tuple, just an integer: {}", not_a_tuple);

    // Function returning a tuple
    let result = return_tuple();
    println!("Function returned tuple: {:?}", result);
    let (a, b, c) = return_tuple();
    println!("Destructured returned tuple: a = {}, b = {}, c = {}", a, b, c);
}
