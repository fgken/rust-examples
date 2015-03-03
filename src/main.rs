fn main() {
    /* Hello world */
    println!("Hello world!");

    variable_bindings();

    if_expression();

    print_number(5);

    print_sum(2, 5);

    println!("{} increment-> {}", 1, increment(1));

    //diverges();
    //println!("never reached here");

    compound_data_types();

    structs();

    tuple_struct();

    ordering_func();
}

// Enum

use std::cmp::Ordering;

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn ordering_func(){
    let x = 5;
    let y = 10;

    let ordering = cmp(x, y);

    if ordering == Ordering::Less {
        println!("less");
    } else if ordering == Ordering::Greater {
        println!("greater");
    } else {
        println!("equal");
    }
}

enum Character {
    Digit(i32),
    Other,
}

fn enum_func() {
    let ten = Character::Digit(10);
    let five = Character::Digit(5);

    // Error: `*` is not implemented for type `Character`
    //let fifty = ten * five;

    // Error: `<=` is not implemented for type `Character`
    //let five_is_smaller = file <= ten;

    // Error: `==` is not implemented for type `Character`
    //let five_equals_ten = file == ten;
}

// Tuple Struct
fn tuple_struct() {
    // newtype
    struct Inches(i32);

    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}

// PointInSpace not Point_In_Space or point_in_space
struct Point {
    x: i32,
    y: i32,
}

fn structs() {
    let origin = Point { x: 0, y: 0 };

    println!("The Origin is at ({}, {})",
                origin.x, origin.y);
}

fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

fn compound_data_types(){
    // Tuples
    let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");

    let mut x = (1, 2);
    let y = (3, 4);
    x = y;

    let (x, y) = next_two(5);
    println!("(x, y) = ({}, {})", x, y);
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
    println!("Hello, {}!", name);
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

