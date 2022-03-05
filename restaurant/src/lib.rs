/*
    crate
    └── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
    │   ├── take_order
    │   ├── serve_order
    │   └── take_payment
    └── eat_at_restaurant()
*/
#![allow(warnings, unused)]

mod front_of_house;

pub use crate::front_of_house::hosting;
use crate::hosting::hosting::add_to_waitlist;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
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

    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
}

fn serve_order() {}
fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    // front_of_house::hosting::add_to_waitlist();
    // with the use key word above, we have brought the hosting module into scope
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
