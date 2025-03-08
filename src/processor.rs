use mongodb::Collection;
use mongodb::bson::doc;
use rust_tradier::account::Order;
use rust_tradier::account::{self, OrdersResponse};
use std::env;

use crate::mongo;
use crate::prep::{Decorate, Uniqueness, Wrapper};

pub(crate) struct Processor {
    db: mongodb::Database,
    coll_orders: mongodb::Collection<Wrapper<Order>>,
}

impl Processor {
    pub async fn new() -> Processor {
        let db = mongo::db("journey").await;
        let coll_orders: Collection<Wrapper<Order>> = mongo::collection(&db, "order");
        Processor { db, coll_orders }
    }

    pub async fn fetch_orders(&self) -> Option<Vec<Order>> {
        let account_id = env::var("TRADIER_ACCOUNT_ID")
            .expect("Required TRADIER_ACCOUNT_ID environment variable was not found");
        let orders = account::fetch_orders(&account_id).await.unwrap();
        // println!("{}", serde_json::to_string_pretty(&orders).unwrap());
        orders
    }

    pub async fn proc_orders(&self) {
        let response = self.fetch_orders().await;
        if let Some(orders) = response {
            for order in orders {
                self.proc_order(order).await;
            }
        } else {
            println!("No orders found");
        }
    }

    pub async fn proc_order(&self, order: Order) {
        let filter = doc! { "uniqueness": order.uniqueness() };
        let existing = self.coll_orders.find_one(filter).await;
        if let Ok(Some(_)) = existing {
            println!("Order {} already exists in db.", order.uniqueness());
            return;
        }
        let wrapped = order.decorate();
        mongo::insert(&self.coll_orders, wrapped).await;
    }
}
