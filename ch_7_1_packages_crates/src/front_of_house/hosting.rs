pub struct Customer {
    pub name: String,
    pub order_id: String,
}
impl Customer {
    pub fn get_customer(name: &str) -> Customer {
        Customer {
            name: String::from(name),
            order_id: String::from("1"),
        }
    }
}

pub fn add_to_waitlist(customer: &Customer) -> i32 {
    1
}
pub fn seat_at_table() {}
