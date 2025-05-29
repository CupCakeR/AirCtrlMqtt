use crate::config::Config;
use crate::homeassistant::HomeAssistantDiscovery;
use chrono::{DateTime, Utc};
use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use std::thread;
use std::time::{Duration, Instant};

pub struct MqttClient {
    client: Client,
    topic: String,
    ha_discovery_prefix: String,
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
            ha_discovery_prefix: config.ha_discovery_prefix.clone(),
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

    pub fn publish_discovery(&self, object_id: &String) {
        println!("Publishing Home Assistant discovery configuration...");

        let ha_discovery = HomeAssistantDiscovery::new(
            object_id.clone(),
            self.topic.clone(),
            self.ha_discovery_prefix.clone(),
        );

        let (topic, payload) = ha_discovery.get_discovery_config();
        if let Err(e) = self.client.publish(&topic, QoS::AtLeastOnce, true, payload) {
            eprintln!("Failed to publish discovery config to {}: {}", topic, e);
        } else {
            println!("Published discovery config to: {}", topic);
        }
    }

    pub fn clone(&self) -> Self {
        MqttClient {
            client: self.client.clone(),
            topic: self.topic.clone(),
            ha_discovery_prefix: self.ha_discovery_prefix.clone(),
        }
    }
}
