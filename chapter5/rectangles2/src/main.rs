// The #[derive(Debug) helps with printing out debug info]
// Debug is a 'Trait'
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This is nice! Adding methods to the rectangle struct
impl Rectangle {

    /*
    Rust knows the type of self is Rectangle as its in impl Rectangle
    Calculate area of the rectangle

    & is in front of self because area borrows self immutably.

    we only need to "read" data so that why borrow.

    if I needed to write to the struct I would use &mut self
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle{
        height: 50,
        width: 30,
    };

    println!(
        "The areas of the rect is {} sq pixels", 
        rect1.area()
    );

    // just a reminder to me that main() still owns rect1
    println!("{:#?}", rect1);
}




