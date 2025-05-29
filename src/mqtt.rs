use crate::config::Config;
use chrono::{DateTime, Utc};
use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use std::thread;
use std::time::{Duration, Instant};

pub struct MqttClient {
    client: Client,
    topic: String,
}

impl MqttClient {
    pub fn new(config: &Config) -> Result<(Self, Connection), Box<dyn std::error::Error>> {
        let mut mqttoptions =
            MqttOptions::new(&config.client_id, &config.mqtt_host, config.mqtt_port);
        mqttoptions.set_keep_alive(Duration::from_secs(60));

        if let (Some(username), Some(password)) = (&config.mqtt_username, &config.mqtt_password) {
            mqttoptions.set_credentials(username, password);
        }

        let (client, connection) = Client::new(mqttoptions, 10);

        let mqtt_client = MqttClient {
            client,
            topic: config.topic.clone(),
        };

        Ok((mqtt_client, connection))
    }

    pub fn validate_connection(
        mut connection: Connection,
    ) -> Result<Connection, Box<dyn std::error::Error>> {
        println!("MQTT: Validating connection to broker...");
        let timeout = Duration::from_secs(10);
        let start = Instant::now();

        while start.elapsed() < timeout {
            match connection.iter().next() {
                Some(Ok(Event::Incoming(Packet::ConnAck(_)))) => {
                    println!("MQTT: Connection validated successfully");
                    return Ok(connection);
                }
                Some(Err(e)) => {
                    return Err(format!("MQTT connection failed: {}", e).into());
                }
                _ => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }

        Err("MQTT connection timeout - broker unreachable".into())
    }

    pub fn start_connection_thread(mut connection: Connection) {
        thread::spawn(move || {
            let mut connected = false;
            for notification in connection.iter() {
                match notification {
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                        if !connected {
                            println!("MQTT: Connected successfully");
                            connected = true;
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("MQTT connection error: {}", e);
                        if connected {
                            println!("MQTT: Connection lost, attempting to reconnect...");
                            connected = false;
                        }
                    }
                }
            }
        });
    }

    pub fn publish_sensor_data(
        &self,
        time: DateTime<Utc>,
        co2: u16,
        temperature: f32,
        humidity: f32,
    ) {
        let payload = format!(
            r#"{{"time":"{}","co2":{},"temperature":{},"humidity":{}}}"#,
            time.to_rfc3339(),
            co2,
            temperature,
            humidity
        );

        if let Err(e) = self
            .client
            .publish(&self.topic, QoS::AtLeastOnce, false, payload)
        {
            eprintln!("Failed to publish MQTT message: {}", e);
        }
    }

    pub fn clone(&self) -> Self {
        MqttClient {
            client: self.client.clone(),
            topic: self.topic.clone(),
        }
    }
}
