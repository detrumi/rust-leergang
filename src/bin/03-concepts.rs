#![allow(dead_code, unused_assignments, unused_variables)]

fn main() {
    let foo = ("The answer", 42);
    // foo.1 += 1; // Not OK

    const MAX: u32 = 100_000;

    let f = 2;
    let mut f = 3; // Shadow
    f = 4; // Update
    let f = 5; // Shadow

    let a: [i32; 3] = [1, 2, 3];
    let b: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];

    let bar = if if true { true } else { false } {
        // ...
    };

    assert!(12.3 - fahrenheit_to_celsius(celsius_to_fahrenheit(12.3)) < 0.001);
    for i in 0..=10 {
        println!("Fib {}: {}", i, fibonacci(i));
    }
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 0;
    for _ in 0..n {
        let old_b = b;
        b += a;
        a = old_b;
    }
    b
}
