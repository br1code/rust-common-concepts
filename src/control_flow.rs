fn main() {
    // if Expressions ---
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.
    // You must be explicit and always provide if with a Boolean as its condition.

    // Handling Multiple Conditions with else if ---
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement ---
    // Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Repetition with Loops ---
    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    loop {
        println!("again!");
    }

    // you can break the loop
    loop {
        print!("just one time");
        break;
    }

    // Returning Values from Loops ---
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job.
    // However, you might need to pass the result of that operation to the rest of your code.
    // To do this, you can add the value you want returned after the break expression you use to stop the loop;
    // that value will be returned out of the loop so you can use it, as shown here:
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // HERE WE RETURN THE VALUE INTO result
        }
    };

    println!("The result is {}", result); // we got 20

    // Conditional Loops with while ---
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // Looping Through a Collection with for ---
    // You COULD use the while construct to loop over the elements of a collection, such as an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // As a more concise alternative, you can use a for loop and execute some code for each item in a collection.
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    // Even in situations in which you want to run some code a certain number of times, most Rustaceans would use a for loop.
    // The way to do that would be to use a Range, which is a type provided by the standard library that generates all
    // numbers in sequence starting from one number and ending before another number.

    // Here’s what the countdown would look like using a for loop and another method I've not yet talked about, rev, to reverse the range:
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}