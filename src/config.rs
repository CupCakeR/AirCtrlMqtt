use std::env;

pub struct Config {
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub client_id: String,
    pub topic: String,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let mqtt_host = env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string());
        let mqtt_port: u16 = env::var("MQTT_PORT")
            .unwrap_or_else(|_| "1883".to_string())
            .parse()
            .expect("MQTT_PORT must be a valid port number");
        let client_id = env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "airctrl_client".to_string());
        let topic = env::var("MQTT_TOPIC").unwrap_or_else(|_| "airctrl/sensors".to_string());
        let mqtt_username = env::var("MQTT_USERNAME").ok();
        let mqtt_password = env::var("MQTT_PASSWORD").ok();

        Config {
            mqtt_host,
            mqtt_port,
            client_id,
            topic,
            mqtt_username,
            mqtt_password,
        }
    }
}
