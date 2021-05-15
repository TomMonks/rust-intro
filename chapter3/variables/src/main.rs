fn main() {

    //default for int is i32
    let x = 5;
    
    let x = x + 1;
    let x = x * 2;
    println!("The valkue of x is {}",x);

    // tuple data type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1)

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Accessed directly: {}", five_hundred);

    // arrays

    // static
    let a = [1, 2, 3, 4, 5];


    // an array that contains the same values for each element
    let a = [3; 5];
    
    println!("the array element at 0 is {}", a[0]);

}

