use manual::orders_from_file;
use processor::Processor;

mod manual;
mod mongo;
mod prep;
mod processor;
mod test;

#[tokio::main]
async fn main() {
    color_backtrace::install();
    let proc = Processor::new().await;
    // proc_from_file(proc, "data/orders.json").await;
    proc.proc_orders().await;
}

async fn proc_from_file(proc: Processor, file: &str) {
    let orders = orders_from_file(file);

    for order in orders {
        proc.proc_order(order).await;
    }
}

// use serde::Deserialize;
// use serde_json;
// use chrono::{DateTime, Utc, TimeZone};

// #[derive(Deserialize, Debug)]
// struct MyStruct {
//     date: DateTime<Utc>,
// }

// fn main() {
//     let json_str = r#"{"date": "2025-03-04T14:49:21.225128500Z"}"#;

//     match serde_json::from_str::<MyStruct>(json_str) {
//         Ok(my_struct) => println!("{:?}", my_struct),
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
