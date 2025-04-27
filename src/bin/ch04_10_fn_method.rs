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
    // Associated function (类似静态方法)
    fn new(start: i32) -> Self {
        Counter { value: start }
    }
    // Method (有 self 参数)
    fn inc(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    // 普通函数
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    print_greeting("Rustacean");

    // 方法与关联函数
    let mut c = Counter::new(10);
    c.inc();
    c.inc();
    println!("Counter value = {}", c.get());
}
