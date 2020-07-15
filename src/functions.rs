fn main() {
    // Functions -----------------------------------

    // Rust code uses snake case as the conventional style for function and variable names.
    // In snake case, all letters are lowercase and underscores separate words.
    do_something();
}

// Rust doesn’t care where you define your functions, only that they’re defined somewhere.
fn do_something() {
    println!("Hi!");
}

// Function Parameters ---

// Functions can also be defined to have parameters, which are special variables that are part of a function’s signature.
// When a function has parameters, you can provide it with concrete values for those parameters.
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
// In function signatures, you must declare the type of each parameter. This is a deliberate decision in
// Rust’s design: requiring type annotations in function definitions means the compiler almost never needs
// you to use them elsewhere in the code to figure out what you mean.

// When you want a function to have multiple parameters, separate the parameter declarations with commas, like this:
fn and_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Functions with Return Values ---
// Functions can return values to the code that calls them. We don’t name return values, but we do declare
// their type after an arrow (->). In Rust, the return value of the function is synonymous with the value
// of the final expression in the block of the body of a function. You can return early from a function by
// using the return keyword and specifying a value, but most functions return the last expression implicitly.
// Here’s an example of a function that returns a value:
fn five() -> i32 {
    5
}
// this is a valid function in Rust.
// The 5 in five is the function’s return value, which is why the return type is i32

// Let’s look at another example:
fn plus_one(x: i32) -> i32 {
    x + 1
}

// plus_one(5) will return 6

// but if we put a semicolon at the end of the line containing x + 1, changing it from
// an expression to a statement, we will get an error.
// fn plus_two(x: i32) -> i32 {
//     x + 2;
// }

// fn plus_two(x: i32) -> i32 {
//     |    --------            ^^^ expected `i32`, found `()`
//     |    |
//     |    implicitly returns `()` as its body has no tail or `return` expression
//     |    x + 2;
//     |         - help: consider removing this semicolon