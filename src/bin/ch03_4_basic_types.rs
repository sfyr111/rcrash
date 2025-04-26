fn main() {
    // Integer types (default: i32)
    let a = -42; // i32 by default
    let b = 42u32; // explicitly u32 using suffix
    let c = 1_000_000_000i64; // explicitly i64 using suffix
    let d: u8 = 255; // explicitly u8 using annotation
    println!("Integer types: a = {} (type: i32, default), b = {} (type: u32, suffix), c = {} (type: i64, suffix), d = {} (type: u8, annotation)", a, b, c, d);

    // Floating-point types (default: f64)
    let x = 3.14; // f64 by default
    let y = 2.71828f32; // explicitly f32 using suffix
    let z: f32 = 1.41421; // explicitly f32 using annotation
    println!("Floating-point types: x = {} (type: f64, default), y = {} (type: f32, suffix), z = {} (type: f32, annotation)", x, y, z);

    // Floating-point precision demo
    let f64_long = 1.2345678901234567890_f64;
    let f32_long = 1.2345678901234567890_f32;
    println!("f64_long = {:.20} (f64, more precision)", f64_long);
    println!("f32_long = {:.20} (f32, less precision)", f32_long);

    // Boolean type (default: bool)
    let is_active = true;
    println!("Boolean type: is_active = {} (type: bool, default)", is_active);

    // Character type (default: char)
    let letter = 'A';
    let emoji = '😄';
    println!("Character type: letter = {} (type: char), emoji = {} (type: char)", letter, emoji);

    // String types
    let s1 = "Hello"; // &str
    let s2 = String::from("World"); // String
    println!("String types: s1 = {} (type: &str), s2 = {} (type: String)", s1, s2);
    let long_str = "123456789012345678901234567890";
    println!("Long string: {} (type: &str, length: {})", long_str, long_str.len());

    // Tuple type (default: inferred)
    let tup = (500, 6.4, 1); // (i32, f64, i32)
    let (tup_x, tup_y, tup_z) = tup;
    println!("Tuple type: tup = ({}, {}, {}), destructured: {}, {}, {}", tup.0, tup.1, tup.2, tup_x, tup_y, tup_z);

    // Array type (default: inferred)
    let arr = [1, 2, 3]; // [i32; 3] by default
    println!("Array type: arr = [{}, {}, {}] (type: [i32; 3], default)", arr[0], arr[1], arr[2]);

    // Slice type
    let slice = &arr[1..]; // &[i32]
    println!("Slice type: slice = {:?} (type: &[i32])", slice);
}
