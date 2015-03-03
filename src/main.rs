fn main() {
    /* Hello world */
    println!("Hello world!");

    variable_bindings();

    if_expression();

    print_number(5);

    print_sum(2, 5);

    println!("{} increment-> {}", 1, increment(1));

    diverges();

    println!("never reached here");
}

/// 'hello' is a function that prints a greeting that is
/// persocalized based on the name given
///
/// # Arguments
/// 
/// * `name` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let name = "Steve";
/// hello(name); // prints "Hello, Steve!"
/// ```
fn hello(name: &str) {
    println!("Hello, {}!", str);
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn increment2(x: i32) -> i32 {
    if 5 < x {
        x
    } else {
        x + 1
    }
}

fn increment(x: i32) -> i32 {
    if 5 < x { return x; }

    x + 1
    // Compile error
    // x + 1;
}

fn print_sum(x: i32, y: i32){
    println!("sum is {}", x+y);
}

// Compile error
//fn print_sum2(x, y){
//    println!("sum is {}", x+y);
//}

fn print_number(x: i32){
    println!("x is: {}", x);
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

