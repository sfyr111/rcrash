fn main() {
    // Integer overflow demo
    let max_u8 = std::u8::MAX;
    println!("The maximum value of u8 is: {}", max_u8);
    let result = max_u8.wrapping_add(1);
    println!("max_u8.wrapping_add(1) = {} (wraps around to 0)", result);

    // Checked addition (returns Option)
    let checked = max_u8.checked_add(1);
    println!("max_u8.checked_add(1) = {:?} (None means overflow)", checked);

    // Overflowing addition (returns (result, overflowed))
    let (overflowing, did_overflow) = max_u8.overflowing_add(1);
    println!("max_u8.overflowing_add(1) = {} (overflowed: {})", overflowing, did_overflow);

    // Overflowing subtraction (returns (result, overflowed))
    let (overflowing_sub, did_overflow_sub) = max_u8.overflowing_sub(1);
    println!("max_u8.overflowing_sub(1) = {} (overflowed: {})", overflowing_sub, did_overflow_sub);

    // Overflow in debug mode (will panic)
    // let overflow = max_u8 + 1; // Uncommenting this line will cause a panic in debug mode

    // Signed integer overflow
    let min_i8 = std::i8::MIN;
    println!("The minimum value of i8 is: {}", min_i8);
    let wrapped = min_i8.wrapping_sub(1);
    println!("min_i8.wrapping_sub(1) = {} (wraps around to 127)", wrapped);
    let (overflowing_i8, did_overflow_i8) = min_i8.overflowing_sub(1);
    println!("min_i8.overflowing_sub(1) = {} (overflowed: {})", overflowing_i8, did_overflow_i8);
    let (overflowing_add_i8, did_overflow_add_i8) = min_i8.overflowing_add(1);
    println!("min_i8.overflowing_add(1) = {} (overflowed: {})", overflowing_add_i8, did_overflow_add_i8);
}
