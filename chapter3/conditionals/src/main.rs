fn main() {
    let num = 6;

    if num % 4 == 0 {
        println!("divisible by 4");

    } else if num % 3 == 0{
        println!("divisible by 3");

    } else {
        println!("not divisible by 4 or 3");
    }

    conditional_selection();
}

fn conditional_selection(){
    let condition = true;
    let number = if condition {5} else {6};
    println!("the value of number is {}", number);
}
