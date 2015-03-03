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

    match_func();

    loop_func();

    string_func();

    array_func();

    vector_func();

    stdin_func();
}

use std::old_io;

fn stdin_func() {
    println!("Type something!");

    let input = old_io::stdin()
                .read_line()
                .ok()
                .expect("Failed to read line");

    println!("{}", input);
}

// Vector
fn vector_func() {
    let mut v = vec![1, 2, 3];

    println!("The length: {}", v.len());
    
    v.push(4);

    println!("The length: {}", v.len());

    let a = [0, 1, 2, 3, 4];
    let middle = &a[3..3];

    for e in middle.iter() {
        println!("middle: {}", e);
    }

}

// Array
fn array_func() {
    let a = [1, 2, 3];  // a: [i32; 3]
    let mut m = [1, 2, 3];  // mut m: [i32; 3]

    let a = [0; 20];    // a: [i32; 20]

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("Array: {}", e);
    }

    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}

// String
fn string_func() {
    // All strings are guaranteed to be validly encoded UTF-8 sequences
    // strings are not null-terminated and can contain null bytes

    // This string binding is reference to this statically 
    // allocated string
    // string slice
    // string slice have fixed size, and cannot be mutated
    let string = "Hello there.";    // string: &str
    println!("{}", string);

    // String
    // `String` is an in-memory string
    let mut s = "Hello".to_string();    // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    takes_slice(&s);
    // takes_slice(s);  // Compile error
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

// Looping

fn loop_func() {
    // for
    let cnt = 5;

    for x in 0..10 {
        println!("for loop: x={}", x);
    }

    for x in 0..cnt {
        println!("for loop2: x={}", x);
    }

    // while
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("while: {}", x);
        if x % 5 == 0 { done = true; }
    }

    x = 6;
    loop {
        x += x - 3;
        println!("while: {}", x);
        if x % 5 == 0 { break; }
    }
}

// Match
fn match_func() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4...10 => println!("4 to 10"),
        _ => println!("something else"),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing => println!("x is missing"),
    }

    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing => println!("y is missing"),
    }

    println!("{}", match cmp(3, 5) {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equal",
    });
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

