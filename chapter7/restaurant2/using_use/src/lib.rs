
// this reads in multiple items into scope from a module
use std::{cmp::Ordering, io};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

}

