/*

Basically associated functions are in the impl block, but don't take &self 
as a parameter.

They are often used as constructor methods!

*/


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
    Associated function: Square.
    */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

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

    /*
    Does other fit inside this rectangle?
    */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(3);

    println!("{:#?}", square1);
    
    println!(
        "The areas of the rect1 is {} sq pixels", 
        rect1.area()
    );
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
