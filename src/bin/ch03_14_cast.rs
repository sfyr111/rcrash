fn main() {
    // Integer to float
    let i = 42;
    let f = i as f64;
    println!("i = {}, f = {}", i, f);

    // Float to integer (truncates toward zero)
    let pi = 3.1415;
    let n = pi as i32;
    println!("pi = {}, n = {}", pi, n);

    // Integer to char (Unicode scalar value)
    let c = 65u8 as char;
    println!("65 as char: {}", c);

    // Char to u32 (Unicode code point)
    let code = 'A' as u32;
    println!("'A' as u32: {}", code);

    // String to integer (parse)
    let s = "123";
    let num: i32 = s.parse().unwrap();
    println!("'{}' parsed to i32: {}", s, num);

    // Integer to string (to_string)
    let n = 456;
    let s = n.to_string();
    println!("{} to string: '{}'", n, s);

    // &str to String and back
    let s1 = "hello";
    let s2 = s1.to_string();
    let s3 = &s2[..];
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    // Option: using as_ref and as_deref
    let s = Some(String::from("world"));
    let s_ref: Option<&str> = s.as_deref();
    println!("Option<String> as_deref: {:?}", s_ref);

    // --- std::convert: From, Into, TryFrom, TryInto ---
    use std::convert::{From, Into, TryFrom, TryInto};

    let s: String = String::from("convert");
    let s2: String = "convert2".into(); // &str -> String
    println!("From/Into: s = {}, s2 = {}", s, s2);

    let n: i32 = i32::from(10u8); // u8 -> i32
    println!("i32::from(10u8): {}", n);

    let res: Result<i32, _> = i32::try_from(100u8); // always Ok
    println!("i32::try_from(100u8): {:?}", res);

    let val: Result<u8, _> = 300i32.try_into(); // will Err (overflow)
    println!("300i32.try_into::<u8>(): {:?}", val);

    // --- std::mem::transmute (DANGEROUS!) ---
    use std::mem::transmute;
    let a: u32 = 0x61626364;
    let b: [u8; 4] = unsafe { transmute(a) };
    println!("transmute u32 to [u8;4]: {:?}", b); // platform-dependent
}
