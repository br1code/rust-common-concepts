fn main() {
    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being
    // specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

    // Scalar Types --------------------------------------
    // A scalar type represents a single value. Rust has four primary scalar types: integers,
    // floating-point numbers, Booleans, and characters.

    // Integer Types ---
    // An integer is a number without a fractional component

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Each variant can be either signed or unsigned and has an explicit size.
    // Signed and unsigned refer to whether it’s possible for the number to be negative or positive

    // the isize and usize types depend on the kind of computer your program is running on: 64 bits
    // if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are
    // generally good choices, and integer types default to i32: this type is generally the fastest, even on 64-bit systems.

    // Floating-Point Types ---
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    let x = 2.0; // f64 default

    let y: f32 = 3.0; // f32

    // Numeric Operations ---

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // The Boolean Type ---
    // As in most other programming languages, a Boolean type in Rust has two possible values: true and false.
    // Booleans are one byte in size.
    let t = true;

    let f: bool = false; // with explicit type annotation

    // The Character Type ---
    // Rust’s char type is the language’s most primitive alphabetic type, and the following code shows one way to use it.
    // (Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means
    // it can represent a lot more than just ASCII.

    // Compound Types -----------------------------------------------------
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // The Tuple Type ---
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // We create a tuple by writing a comma-separated list of values inside parentheses.
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t
    // have to be the same. We’ve added optional type annotations in this example:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to
    // take tup and turn it into three separate variables, x, y, and z. This is called destructuring, because
    // it breaks the single tuple into three parts.

    // In addition to destructuring through pattern matching, we can access a tuple element directly by
    // using a period (.) followed by the index of the value we want to access. For example:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // The Array Type ---
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element
    // of an array must have the same type. Arrays in Rust are different from arrays in some other languages
    // because arrays in Rust have a FIXED length, like tuples.

    let my_array = [1, 2, 3, 4, 5];
    // Arrays are useful when you want your data allocated on the stack rather than the heap or
    // when you want to ensure you always have a fixed number of elements

    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided
    // by the standard library that is allowed to grow or shrink in size.
    // If you’re unsure whether to use an array or a vector, YOU SHOULD probably use a vector.

    // An example of when you might want to use an array rather than a vector is
    // in a program that needs to know the names of the months of the year.
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // You would write an array’s type by using square brackets, and within the brackets include the type of
    // each element, a semicolon, and then the number of elements in the array, like so:
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

    // Writing an array’s type this way looks similar to an alternative syntax for initializing an array:
    // if you want to create an array that contains the same value for each element, you can specify the initial value,
    // followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let filled = [3; 5]; // the same as writing let filled = [3, 3, 3, 3, 3]

    // Accessing Array Elements
    // An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // 1
    let second = a[2]; // 2

    // Invalid Array Element Access
    // What happens if you try to access an element of an array that is past the end of the array?
    // Say you change the example to the following code, which will compile but exit with an error when it runs:
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    // Running this code using cargo run produces the following result:
    // panicked at 'index out of bounds: the len is 5 but the index is 10'

    // The compilation didn’t produce any errors, but the program resulted in a runtime error and didn’t exit successfully.
    // When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length.
    // If the index is greater than or equal to the array length, Rust will panic.
}