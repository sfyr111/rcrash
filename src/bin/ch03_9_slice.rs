fn main() {
    // Array slice
    let arr = [10, 20, 30, 40, 50];
    let slice_all = &arr[..];
    let slice_part = &arr[1..4];
    println!("Array: {:?}", arr);
    println!("Full slice: {:?}", slice_all);
    println!("Partial slice [1..4]: {:?}", slice_part);

    // Slices are references to a contiguous sequence
    println!("First element of slice: {}", slice_part[0]);
    println!("Length of slice: {}", slice_part.len());

    // Modifying array through mutable slice
    let mut arr2 = [1, 2, 3, 4, 5];
    let slice_mut = &mut arr2[2..];
    slice_mut[0] = 99;
    println!("Modified arr2 via slice: {:?}", arr2);

    // String slice
    let s = String::from("Hello, Rustaceans!");
    let hello = &s[0..5];
    let rest = &s[7..];
    println!("String: {}", s);
    println!("First word: {}", hello);
    println!("Rest of string: {}", rest);
}
