use std::{fs::File, io::BufReader};

use rust_tradier::account::{Order, OrdersResponse, value_to_orders};

use crate::{prep::Decorate, test::Test};

pub(crate) fn orders_from_file(path: &str) -> Vec<Order> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    value_to_orders(serde_json::from_reader(reader).unwrap()).unwrap()
}

pub(crate) fn test() {
    let test = Test {
        id: 1,
        value: "test".to_string(),
    };
    let wrapper = test.decorate();
    println!("{:?}", wrapper);
}
