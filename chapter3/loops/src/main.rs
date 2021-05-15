fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
        
    };

    println!("The result is {}", result);

    while_loop();
    for_loop();
    for_loop2();
}



fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -=1;
    }

    println!("LIFT OFF!!!!")
}


fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_loop2(){
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("lift off!!!!")
}