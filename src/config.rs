use std::env;

pub struct Config {
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub client_id: String,
    pub topic: String,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
    pub ha_discovery_enabled: bool,
    pub ha_discovery_prefix: String,
    pub object_id: String,
}

impl Config {
    pub fn from_env() -> Self {
        let mqtt_host = env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".into());
        let mqtt_port: u16 = env::var("MQTT_PORT")
            .unwrap_or_else(|_| "1883".into())
            .parse()
            .expect("MQTT_PORT must be a valid port number");
        let client_id = env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "airctrl_client".into());
        let topic = env::var("MQTT_TOPIC").unwrap_or_else(|_| "airctrl/sensors".into());
        let mqtt_username = env::var("MQTT_USERNAME").ok();
        let mqtt_password = env::var("MQTT_PASSWORD").ok();
        let ha_discovery_enabled = env::var("HA_DISCOVERY_ENABLED")
            .unwrap_or_else(|_| "true".into())
            .parse()
            .unwrap_or(true);
        let ha_discovery_prefix =
            env::var("HA_DISCOVERY_PREFIX").unwrap_or_else(|_| "homeassistant".into());
        let object_id = env::var("HA_OBJECT_ID").unwrap_or_else(|_| "AirCtrlMqtt_7d269530".into());

        Config {
            mqtt_host,
            mqtt_port,
            client_id,
            topic,
            mqtt_username,
            mqtt_password,
            ha_discovery_enabled,
            ha_discovery_prefix,
            object_id,
        }
    }
}
