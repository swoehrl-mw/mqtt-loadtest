use rumqttc::v5::{AsyncClient, EventLoop, MqttOptions, ConnectionError};
use rumqttc::{TlsConfiguration, Transport};

fn init_tls_transport(ca_cert_path: &str) -> Transport {
    let ca_cert = std::fs::read_to_string(&ca_cert_path).expect("Could not read CA cert file");
    Transport::Tls(TlsConfiguration::Simple {
        ca: ca_cert.into_bytes(),
        alpn: None,
        client_auth: None,
    })
}

pub type MqttClient = AsyncClient;

pub async fn create_client() -> (AsyncClient, EventLoop) {
    let clientid = std::env::var("MQTT_CLIENTID").unwrap_or_else(|_| "test".to_string());
    let host = std::env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("MQTT_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .unwrap();
    let mut mqttoptions = MqttOptions::new(clientid.clone(), host, port);
    mqttoptions
        .set_connection_timeout(20)
        .set_clean_session(true)
        .set_inflight(1000)
        ;

    if let Ok(ca_cert_path) = std::env::var("MQTT_CA_CERT") {
        mqttoptions.set_transport(init_tls_transport(&ca_cert_path));
    }

    if let Ok(username) = std::env::var("MQTT_USERNAME") {
        if let Ok(password) = std::env::var("MQTT_PASSWORD") {
            mqttoptions.set_credentials(username, password);
        }
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 1000_usize);

    // Do one poll to check if the connection is established
    match eventloop.poll().await {
        Ok(_packet) => {
            println!("Connected with clientid {}", clientid);
        }
        Err(err) => {
            if let ConnectionError::NotConnAck(..) = err {
                eventloop.poll().await.unwrap();
            } else {
                panic!("Failed to connect to mqtt: {}", err);
            }
        }
    };
    (client, eventloop)
}

pub async fn start_eventloop(mut eventloop: EventLoop) {
    tokio::spawn(async move {
        loop {
            let poll_result = eventloop.poll().await;
            match poll_result {
                Err(err) => {
                    println!("Err: {}", err);
                }
                _ => (),
            }
        }
    });
}