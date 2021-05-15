// The #[derive(Debug) helps with printing out debug info]
// Debug is a 'Trait'
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The areas of the rect is {} sq pixels", 
        area(&rect1));

    // just a reminder to me that main() still owns rect1
    println!("{:#?}", rect1);
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
