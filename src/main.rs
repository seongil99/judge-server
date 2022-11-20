use publisher::Payload;

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

    let addr = "amqp://rabbitmq:5672/%2f";

    // let consume_channel = consumer::create_channel(addr);
    let publish_channel = publisher::create_channel(addr);
    let result = String::from("test");

    // consumer::consume(consume_channel);
    let msg = Payload {
        answer_id: 123,
        memory: 1,
        time: 2,
        result: result
    };

    publisher::publish(publish_channel, msg);
}
