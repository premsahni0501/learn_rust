pub mod back_of_house;
pub mod front_of_house;
use crate::back_of_house::Appetizer;
use crate::back_of_house::BreakFast;
use crate::front_of_house::hosting as Hosting;
use crate::front_of_house::serving as Serving;

pub fn eat_at_restaurant(customer: &Hosting::Customer) {
    Hosting::add_to_waitlist(customer);
    Hosting::seat_at_table();
    Serving::take_order();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = BreakFast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

fn deliver_order() {}
