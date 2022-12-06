use crate::cli::SubscriberArgs;
use crate::mqtt::create_client;
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::mqttbytes::{Filter, QoS};
use rumqttc::v5::Event;
use std::time::{Duration, Instant};

pub async fn run(_args: SubscriberArgs) {
    // Connect to MQTT broker
    let (client, mut eventloop) = create_client().await;

    // Subscribe to topics
    let topic = std::env::var("MQTT_TOPIC").unwrap_or_else(|_| "devices/#".to_string());
    let topics: Vec<Filter> = topic
        .split(',')
        .map(|el| Filter::new(el, QoS::AtLeastOnce))
        .collect();
    client.subscribe_many(topics).await.unwrap();
    // Poll once to make sure subscribe is sent
    eventloop.poll().await.unwrap();

    // Init stats
    let mut count: u64 = 0;
    let mut sum: u64 = 0;
    let mut last_stats = Instant::now();

    // Receive messages in a loop
    loop {
        tokio::select! {
            poll_result = eventloop.poll() => {
                match poll_result {
                    Ok(event) => if let Event::Incoming(incoming) = event {
                        if let Packet::Publish(..) = *incoming {
                            count += 1;
                            sum += 1;
                        } else if let Packet::SubAck(..) = *incoming {
                            println!("Subscribed to MQTT topics successfully");
                        }
                    },
                    Err(err) => {
                        println!("Err: {}", err);
                    },
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(1)) => {
            }
        };

        if last_stats.elapsed().as_secs() > 2 {
            let duration = last_stats.elapsed().as_millis();
            let speed = count * 1000 / duration as u64;
            println!("Current speed: {} messages/s. Sum received: {}", speed, sum);
            count = 0;
            last_stats = Instant::now();
        }
    }
}
