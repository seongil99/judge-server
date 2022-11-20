use std::cmp::max;

mod consumer;
mod executor;
mod filter;
mod judge;
mod publisher;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let addr = "amqp://judge-server-rabbitmq-1:5672/%2f";

    let consume_channel = consumer::create_channel(addr);
    let publish_channel = publisher::create_channel(addr);

    consumer::consume(consume_channel);
}
