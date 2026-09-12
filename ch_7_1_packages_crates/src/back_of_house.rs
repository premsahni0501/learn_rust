pub enum Appetizer {
    Soup,
    Salad,
}
pub struct BreakFast {
    pub toast: String,
    seasonal_fruit: String,
}
impl BreakFast {
    pub fn summer(toast: &str) -> BreakFast {
        BreakFast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}
fn cook_order() {}
