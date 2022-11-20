use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use tracing::info;

use crate::{
    executor::{self, Problem},
    judge::{self, Status},
};

pub fn create_channel(addr: &str) -> lapin::Channel {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| addr.into());
    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");
        info!(state=?conn.status().state());

        let queue = channel
            .queue_declare(
                "to_rust",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");
        info!(state=?conn.status().state());
        info!(?queue, "Declared queue");

        channel
    })
}

pub fn consume(chan: lapin::Channel) {
    async_global_executor::block_on(async {
        let mut consumer = chan
            .basic_consume(
                "to_rust",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        while let Some(delivery) = consumer.next().await {
            info!(message=?delivery, "received message");
            if let Ok(delivery) = delivery {
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
                let payload = &delivery.data;
                let problem = Problem::from_payload(&payload);
                problem.write_code_file();
                executor::main();
                judge::main(Status::Accepted);
            }
        }
    })
}
