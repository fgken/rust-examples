fn main() {
    /* Hello world */
    println!("Hello world!");

    variable_bindings();

    if_statement();

}

fn variable_bindings() {
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

fn if_expression() {    // not if statement
    // "Rust is an expression-based language"

    let x = 5;

    if x == 5 {
        println!("x is 5!");
    }

    // expression statement
    let y = if x == 5 { 10 } else { 15 };  // y: i32
    // let y = if x == 5 { 10; } else { 15; };  // Compile error

    println!("y is {}!", y);

    // let x = (let y = 5): // Compile error
    // let can only begin a statement


}
