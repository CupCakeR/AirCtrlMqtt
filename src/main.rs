mod config;
mod homeassistant;
mod mqtt;

use aircontrol::AirControl;
use config::Config;
use hidapi::HidApi;
use mqtt::MqttClient;

const VENDOR_ID: u16 = 0x04d9;
const PRODUCT_ID: u16 = 0xa052;

fn main() {
    let config = Config::from_env();

    let api = HidApi::new()
        .map_err(|_| "Failed to create HID API instance")
        .unwrap();
    let device = api
        .open(VENDOR_ID, PRODUCT_ID)
        .map_err(|_| "Failed to open device")
        .unwrap();

    let (mqtt_client, connection) = MqttClient::new(&config).expect("Failed to create MQTT client");

    let validated_connection =
        MqttClient::validate_connection(connection).expect("Failed to connect to MQTT broker");

    MqttClient::start_connection_thread(validated_connection);

    // discovery
    if config.ha_discovery_enabled {
        mqtt_client.publish_discovery(&config.object_id);
    } else {
        println!("Home Assistant discovery disabled, skipping...");
    }

    let mut air_control = AirControl::new().expect("Failed to initialize the AirControl interface");

    let mqtt_client_clone = mqtt_client.clone();
    air_control.register_callback(Box::new(move |time, co2, temperature, humidity| {
        println!(
            "{} - CO2: {} ppm, Temp: {}°C, Humidity: {}%",
            time, co2, temperature, humidity
        );
        mqtt_client_clone.publish_sensor_data(time, co2, temperature, humidity);
    }));

    air_control.start_monitoring();

    loop {}
}
