fn main() {
    let n = 10;
    let array = [1, 4, 3, 2, 2];
    let array2 = vec![true; n];
    for (i, &value) in array.iter().enumerate() {
        println!("{}, {}", i, value);
    }

    for (i, &value) in array2.iter().enumerate() {
        println!("{}, {}", i, value);
    }


    

}
