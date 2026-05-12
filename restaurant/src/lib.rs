/// Modules of restaurant
// Crate root module at top as default

// use rand::Rng;     // bring Rng trait into scope

use rand::{Rng,CryptoRng, ErrorKind::Transient};     // bring Rng, CryptoRng traits and ErrorKind::Transient into scope

use std::io::{self,Write};     // bring io and Write into scope}



mod front_of_house;     // declare front_of_house module  (defined in src/front_of_house.rs)

pub use crate::front_of_house::hosting;     // bring hosting module into scope

pub fn eat_at_restaurant() {

    let _secret_number = rand::thread_rng().gen_range(1, 100);     // use Rng trait to generate random number
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();


    front_of_house::serving::take_order();     

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // seasonal_fruit = String::from("blueberries");     // error: seasonal_fruit is private

    let order1 = back_of_house::Appetizer::Soup;     // enum variants are public by default
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,     // public field
        seasonal_fruit: String,   // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {     // public function
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();     // super to access parent module
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}