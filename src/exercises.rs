fn main() {
    println!("Learning common programming concepts in Rust");
    let fahrenheit = 45.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    let celsius = 60.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius,fahrenheit);

    println!("The fibonacci value of 1 is: {}", fib(1));
    println!("The fibonacci value of 3 is: {}", fib(3));
    println!("The fibonacci value of 5 is: {}", fib(5));
    println!("The fibonacci value of 10 is: {}", fib(10));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fib(n: u32) -> u32 {
    match n {
        0 => n,
        1 => n,
        _ => fib(n - 1) + fib(n - 2)
    }
}
