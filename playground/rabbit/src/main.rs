use amiquip::{Connection, Exchange, Publish, Queue, QueueDeclareOptions, Result};
use std::time::Duration;

fn main() -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    println!("{:?}", connection.server_properties());
    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("My_Queue", QueueDeclareOptions::default())?;
    // Get a handle to the direct exchange on our channel.

    // Publish a message to the "hello" queue.
    loop {
        
        exchange.publish(Publish::new("hello there".as_bytes(), "hello"))?;
        std::thread::sleep(Duration::from_millis(20));
    }
}
