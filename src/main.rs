use std::env;
use mongodb::Collection;
use rust_tradier::account;
use rust_tradier::account::Order;

use prep::{Test, Decorate};
mod mongo;
mod prep;

#[tokio::main]
async fn main() {
    let test = Test { id: 1, value: "test".to_string() };
    let wrapper = test.decorate();
    // let wrapper = Wrapper { _id: uuid7::uuid7(), ts: Utc::now(), native_id: test.native_id(), obj: test };
    println!("{:?}", wrapper);
    if true {
        return;
    }

    let account_id = env::var("TRADIER_ACCOUNT_ID")
        .expect("Required TRADIER_ACCOUNT_ID environment variable was not found");
    let orders = account::fetch_orders(&account_id).await.unwrap();
    // println!("{}", serde_json::to_string_pretty(&orders).unwrap());
    println!("{:?}", orders);

    if let Some(orders) = orders {
        let database = mongo::db("journey").await;
        let collection: Collection<Order> = mongo::collection(&database, "orders");
        // TODO: Add decoration, hash, and check for duplicates
        mongo::insert_many(&collection, orders).await;
    } else {
        println!("No orders found");
    }
}
