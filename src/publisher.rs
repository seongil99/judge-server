use lapin::{
    message::{BasicReturnMessage, Delivery},
    options::*,
    protocol::{AMQPErrorKind, AMQPSoftError},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use serde_json::json;

use crate::judge::JudgeResult;

const QUEUE_NAME: &str = "to_spring";
const QUEUE_ADDR: &str = "ampq://rabbitmq:5672/%2f";

pub fn create_channel(addr: &str) -> lapin::Channel {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| addr.into());
    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");

        let queue = channel
            .queue_declare(
                QUEUE_NAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");

        channel
    })
}

pub fn publish(chan: lapin::Channel, msg: JudgeResult) {
    async_global_executor::block_on(async {
        let queue = chan
            .queue_declare(
                QUEUE_NAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");

        chan.confirm_select(ConfirmSelectOptions::default())
            .await
            .expect("confirm_select");
        let confirm = chan
            .basic_publish(
                "",
                QUEUE_NAME,
                BasicPublishOptions::default(),
                serde_json::to_string(&msg).unwrap().as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("basic_publish")
            .await // Wait for this specific ack/nack
            .expect("publisher-confirms");
        confirm.is_ack();
    });
}
