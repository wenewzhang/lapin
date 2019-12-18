use futures::future::Future;
use lapin_futures as lapin;
use crate::lapin::{BasicProperties, Client, ConnectionProperties};
use crate::lapin::options::{BasicPublishOptions, QueueDeclareOptions, ConfirmSelectOptions};
use crate::lapin::types::FieldTable;
use log::info;

fn main() {
  env_logger::init();

  let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

  futures::executor::spawn(
   Client::connect(&addr, ConnectionProperties::default()).and_then(|client| {
      // create_channel returns a future that is resolved
      // once the channel is successfully created
      client.create_channel()
    }).and_then(|channel| {
      let id = channel.id();
      info!("created channel with id: {}", id);

      // we using a "move" closure to reuse the channel
      // once the queue is declared. We could also clone
      // the channel
      channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::default()).and_then(move |_| {
        info!("channel {} declared queue {}", id, "hello");
        // channel
        //     .confirm_select(ConfirmSelectOptions::default())
        //     .wait()
        //     .expect("confirm_select");
        channel.basic_publish("", "hello", b"hello from tokio".to_vec(), BasicPublishOptions::default(), BasicProperties::default())
      })
    })
  ).wait_future().expect("runtime failure");
}
