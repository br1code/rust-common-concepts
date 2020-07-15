fn main() {
    // Immutable vs mutable ---------------------------

    // When a variable is immutable, once a value is bound to a name, you can’t change that value.
    let x = 5;

    println!("The value of x is: {}", x);

    // x = 6; ERROR: Cannot assign twice to immutable variable

    //  However, you still have the option to make your variables mutable using the mut keyword
    let mut y = 5;

    println!("The value of y is: {}", y);

    y = 6;

    println!("The value of y is: {}", y);

    // Variables vs Constants ---------------------------

    // First, you aren’t allowed to use mut with constants. Constants aren’t just
    // immutable by default—they’re always immutable.

    // You declare constants using the const keyword instead of the let keyword, and the type
    // of the value must be annotated.

    // Constants can be declared in any scope, including the global scope, which makes them useful
    // for values that many parts of code need to know about.

    // The last difference is that constants may be set only to a constant expression, not the result
    // of a function call or any other value that could only be computed at runtime.

    // Here’s an example of a constant declaration where the constant’s name is MAX_POINTS
    // and its value is set to 100,000.
    const MAX_POINTS: u32 = 100_000;

    // Shadowing -------------------------------------------

    // you can declare a new variable with the same name as a previous variable, and the new
    // variable shadows the previous variable

    //  We can shadow a variable by using the same variable’s name and repeating the
    // use of the let keyword as follows:
    let z = 5;

    let z = z + 1;

    let z = z * 2;

    println!("The value of z is: {}", z);

    // This program first binds x to a value of 5. Then it shadows x by repeating let x =, taking
    // the original value and adding 1 so the value of x is then 6. The third let statement also
    // shadows x, multiplying the previous value by 2 to give x a final value of 12. When we run
    // this program, it will output The value of z is: 12

    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
    // if we accidentally try to reassign to this variable without using the let keyword.
    // By using let, we can perform a few transformations on a value but have the variable
    // be immutable after those transformations have been completed.

    // The other difference between mut and shadowing is that because we’re effectively creating a
    // new variable when we use the let keyword again, we can change the type of the value but reuse
    // the same name. For example, say our program asks a user to show how many spaces they want between
    // some text by inputting space characters, but we really want to store that input as a number:
    let spaces = "   "; // string
    let spaces = spaces.len(); // usize

    //  if we try to use mut for this, as shown here, we’ll get a compile-time error:
    let mut spaces = "   ";
    // spaces = spaces.len(); ERR: expected `&str`, found `usize`
}