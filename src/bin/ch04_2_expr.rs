fn main() {
    // 1. Arithmetic expressions
    let sum = 1 + 2 * 3;
    println!("sum = {}", sum);

    // 2. Block expression (returns last value)
    let x = {
        let a = 10;
        let b = 20;
        a + b // no semicolon means this value is returned
    };
    println!("block expr x = {}", x);

    // 3. If expression (not statement!)
    let cond = true;
    let val = if cond { 42 } else { 0 };
    println!("if expr val = {}", val);

    // 4. Match expression (returns a value)
    let n = 3;
    let msg = match n {
        1 => "one",
        2 | 3 => "two or three",
        _ => "other",
    };
    println!("match expr msg = {}", msg);

    // 5. Function call expression
    fn square(n: i32) -> i32 { n * n }
    let sq = square(4);
    println!("square(4) = {}", sq);

    // 6. Tuple, array, struct, enum construction are also expressions
    let tup = (1, 2);
    let arr = [1, 2, 3];
    struct Point { x: i32, y: i32 }
    let p = Point { x: 3, y: 4 };
    enum Color { Red, Green, Blue }
    let c = Color::Green;
    println!("tup = {:?}, arr = {:?}, p = ({} {}), enum = {:?}", tup, arr, p.x, p.y, c as u8);

    // 7. Loop expressions (can return values with break)
    let mut i = 0;
    let res = loop {
        i += 1;
        if i > 5 { break i * 10; }
    };
    println!("loop expr res = {}", res);
}
