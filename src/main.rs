fn main() {
    /* Hello world */
    println!("Hello world!");

    /* Variable bindings */
    let x = 5;
    
    // pattern matching
    let (y, z) = (1, 2);    

    // type inference
    let a = 5;      // a: i32
    let a: i32 = 5; // i8, i16, i32, i64, u8, u16 ...
    
    // By default, bindings are immutable
    // a = 10;     // Compile error!
    let mut a = 5;
    a = 10;

    println!("The value of a is: {}", a);
}
