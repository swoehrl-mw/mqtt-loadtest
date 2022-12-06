use crate::cli::PublisherArgs;
use crate::mqtt::{create_client, start_eventloop, MqttClient};
use crate::util::CyclicRangeIterator;
use rumqttc::v5::mqttbytes::QoS;
use std::time::Instant;


pub async fn run(_args: PublisherArgs) {
    // Connect to MQTT broker
    let (client, eventloop) = create_client().await;

    // Start eventloop that handles actual sending of messages in the backgorund
    start_eventloop(eventloop).await;

    // Prepare stats
    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut last_stats = Instant::now();

    // Iterate through deviceids and send messages
    let iterator = CyclicRangeIterator::new(1, 1000);
    for device_id in iterator {
        // Send message
        publish_message(device_id, &client).await;
        count += 1;
        sum += 1;

        if last_stats.elapsed().as_secs() > 2 {
            let duration = last_stats.elapsed().as_millis() as u64;
            let speed = count * 1000 / duration;
            println!("Current speed: {} messages/s, sum sent: {}", speed, sum);
            count = 0;
            last_stats = Instant::now();
        }
    }

    client.disconnect().await.unwrap();
}

async fn publish_message(device_id: u64, client: &MqttClient) {
    let topic = format!("devices/{}", device_id);
    let payload = vec![1; 1024];
    client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
        .unwrap();
}
