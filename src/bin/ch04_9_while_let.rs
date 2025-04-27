fn main() {
    // Basic while let with Option
    let mut opt = Some(5);
    while let Some(n) = opt {
        println!("n = {}", n);
        if n == 0 {
            opt = None;
        } else {
            opt = Some(n - 1);
        }
    }

    // while let with Result
    let mut results = vec![Ok(1), Ok(2), Err("fail")].into_iter();
    while let Some(Ok(x)) = results.next() {
        println!("Ok: {}", x);
    }

    // while let with pattern guard
    let mut opt = Some(3);
    while let Some(n) = opt {
        if n % 2 == 0 {
            println!("even: {}", n);
            break;
        }
        println!("odd: {}", n);
        opt = if n > 0 { Some(n - 1) } else { None };
    }
}
