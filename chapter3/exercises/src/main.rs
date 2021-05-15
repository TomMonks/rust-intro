fn main() {
    println!("Hello, world!");

    let deg_f = convert_celsius_to_fahrenheit(39.2);
    println!("Degress fahrenheit = {}", deg_f);
    convert_data();
    let nth_fib = fib_iter(40);
    println!("The 40th fibonnaci number is: {}", nth_fib);
}


// Exercise 1: convert celsius to fahrenheit

fn convert_data(){
    let temps = [39.2, 36.5, 37.3, 41.0];

    for deg_c in temps.iter() {
        let deg_f = convert_celsius_to_fahrenheit(*deg_c);
        println!("Degress fahrenheit = {}", deg_f);
    }
}


fn convert_celsius_to_fahrenheit(deg_celsius: f64) -> f64 {
    (9.0/5.0) * deg_celsius + 32.0
}
    
// exercise 2: generate the nth fibbonaci number

fn fib_iter(n: i64) -> i64 {
    let mut last = 0;
    let mut next = 1;

    for i in (1..n) {
        last = next;
        next = last + next;
    }

    next
}

