use std::env;
use mongodb::Collection;
use rust_tradier::account;
use rust_tradier::account::Order;

mod mongo;

#[tokio::main]
async fn main() {
    let account_id = env::var("TRADIER_ACCOUNT_ID")
        .expect("Required TRADIER_ACCOUNT_ID environment variable was not found");
    let orders = account::fetch_orders(&account_id).await.unwrap();
    // println!("{}", serde_json::to_string_pretty(&orders).unwrap());
    println!("{:?}", orders);

    if let Some(orders) = orders {
        let database = mongo::db("journey").await;
        let collection: Collection<Order> = mongo::collection(&database, "orders");
        mongo::insert_many(&collection, orders).await;
    } else {
        println!("No orders found");
    }
}
