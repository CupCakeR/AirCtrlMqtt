mod config;
mod mqtt;

use aircontrol::AirControl;
use config::Config;
use mqtt::MqttClient;

fn main() {
    let config = Config::from_env();
    let (mqtt_client, connection) = MqttClient::new(&config).expect("Failed to create MQTT client");

    let validated_connection =
        MqttClient::validate_connection(connection).expect("Failed to connect to MQTT broker");

    MqttClient::start_connection_thread(validated_connection);

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
