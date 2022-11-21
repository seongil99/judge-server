use judge::JudgeResult;

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

    let consume_channel = consumer::create_channel(addr);
    let publish_channel = publisher::create_channel(addr);

    consumer::consume(consume_channel);
    let msg: JudgeResult = JudgeResult::new(judge::Status::Accepted, 1, 2, 123);
    publisher::publish(publish_channel, msg);
}
