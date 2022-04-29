// signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
// and the unit type (), whose only possible value is an empty tuple: ()

// Mutability 
//  let _immutable_binding = 1; => in JavaScript  const _immutable_binding = 1;
//  let mut _immutable_binding = 1; => in JavaScript  let _immutable_binding = 1;


fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type: i64 = 12; // Type i64 is inferred from another line
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);
    
    // A mutable variable's value can be changed.
    let mut m = 12; // Mutable `i32`
    println!("{}", m);
    m = 21;
    println!("{}", m);
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let m = true;
}
