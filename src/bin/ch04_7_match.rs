fn main() {
    // Basic match statement
    let n = 2;
    match n {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=6 => println!("four to six"),
        _ => println!("something else"),
    }

    // Match with enum
    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Green;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // Match with Option
    let maybe = Some(42);
    match maybe {
        Some(x) if x > 40 => println!("Large: {}", x),
        Some(x) => println!("Small: {}", x),
        None => println!("None"),
    }

    // Match with struct destructuring
    struct Person {
        name: String,
        age: u8,
    }
    let p = Person { name: "Alice".to_string(), age: 30 };
    match p {
        Person { name, age: 30 } => println!("{} is 30 years old", name),
        Person { name, age } => println!("{} is {} years old", name, age),
    }

    // Match with tuple destructuring
    let point = (0, -2);
    match point {
        (0, y) => println!("On y axis at y = {}", y),
        (x, 0) => println!("On x axis at x = {}", x),
        (x, y) => println!("At ({}, {})", x, y),
    }

    // Match as an expression
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("grade = {}", grade);
}
