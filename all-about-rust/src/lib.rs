fn deliver_order() {}

mod back_of_house;
mod front_of_house;

// the 'use' keyword is a great shortcut for calling for functions
use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    use crate::front_of_house::hosting;

    fn new_function() {
        hosting::add_to_waitlist()
    }
}

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

    let new_order = back_of_house::cook_order();
    let fix_order = back_of_house::fix_incorrect_order();
}
