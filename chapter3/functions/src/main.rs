fn main() {
    println!("Hello, world!");

    another_function(five(), 6);

}

fn another_function(x: i32, y: i32){
    println!("Another function x = {}", x);
    println!("Another function y = {}", y);
}

fn five() -> i32 {
    // rust returns implicitly
    5
}
