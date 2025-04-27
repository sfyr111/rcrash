fn main() {
    // Basic while loop
    let mut n = 3;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }
    println!("Liftoff!");

    // while loop over a collection (using index)
    let arr = [10, 20, 30, 40];
    let mut idx = 0;
    while idx < arr.len() {
        println!("arr[{}] = {}", idx, arr[idx]);
        idx += 1;
    }

    // while let for Option/enum pattern matching
    let mut opt = Some(5);
    while let Some(x) = opt {
        println!("while let: x = {}", x);
        if x == 0 {
            opt = None;
        } else {
            opt = Some(x - 1);
        }
    }
    println!("while let finished");
}
