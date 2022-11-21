use judge::JudgeResult;

mod consumer;
mod executor;
mod filter;
mod judge;
mod publisher;

fn main() {
    let addr = "amqp://rabbitmq:5672/%2f";

    let consume_channel = consumer::create_channel(addr);

    consumer::consume(consume_channel);
}
