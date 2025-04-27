fn main() {
    // Basic if let with Option
    let opt = Some(10);
    if let Some(x) = opt {
        println!("Option contains {}", x);
    } else {
        println!("Option is None");
    }

    // if let with enum
    enum Status {
        Ok(u32),
        Error(String),
    }
    let s = Status::Ok(200);
    if let Status::Ok(code) = s {
        println!("Success: {}", code);
    } else {
        println!("Not Ok");
    }

    // if let with while let (counter)
    let mut opt = Some(3);
    while let Some(n) = opt {
        println!("n = {}", n);
        if n == 0 {
            opt = None;
        } else {
            opt = Some(n - 1);
        }
    }

    // if let else (Rust 1.65+)
    let opt: Option<i32> = None;
    if let Some(x) = opt {
        println!("Found {}", x);
    } else {
        println!("Nothing found");
    }
}
