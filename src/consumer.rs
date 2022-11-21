use futures_lite::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use tracing::info;

use crate::{
    executor::{self, Problem},
    judge::{self, JudgeResult, Status},
    publisher,
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
    let addr = "amqp://rabbitmq:5672/%2f";
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
                problem.write_testcase_file();

                let exe_result = executor::main();

                match exe_result {
                    Ok(result) => {
                        let judge_result = judge::main();
                        let mut judge_status: Status;
                        match judge_result {
                            Ok(judge_result) => {
                                judge_status = judge_result;
                            }
                            Err(e) => {
                                judge_status = Status::SystemError;
                            }
                        }

                        let mut judge_result =
                            JudgeResult::from_result_files(judge_status, problem.answer_id);
                        let judge_result_json = serde_json::to_string(&judge_result).unwrap();

                        info!(?judge_result_json, "judge_result_json");

                        let publish_channel = publisher::create_channel(addr);
                        publisher::publish(publish_channel, judge_result);
                    }
                    Err(e) => {
                        let judge_result =
                            JudgeResult::from_result_files(Status::CompileError, problem.answer_id);
                        info!(?e, "error");

                        let publish_channel = publisher::create_channel(addr);
                        publisher::publish(publish_channel, judge_result);
                    }
                }

                judge::clean();
            }
        }
    })
}
