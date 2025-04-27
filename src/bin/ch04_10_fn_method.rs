fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_greeting(name: &str) {
    println!("Hello, {}!", name);
}

struct Counter {
    value: i32,
}

impl Counter {
    // Associated function (like a static method)
    fn new(start: i32) -> Self {
        Counter { value: start }
    }
    // Method (has self parameter)
    fn inc(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    // Regular function
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    print_greeting("Rustacean");

    // Methods and associated function
    let mut c = Counter::new(10);
    c.inc();
    c.inc();
    println!("Counter value = {}", c.get());
}
