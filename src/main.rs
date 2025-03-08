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
