mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order() {}
}

// the 'use' keyword is a great shortcut for calling for functions
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Absolute path
    add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // here the implementation of the back_of_house mod
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast == String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order_appetizer1 = back_of_house::Appetizer::Soup;
    let order_appetizer2 = back_of_house::Appetizer::Salad;
}
