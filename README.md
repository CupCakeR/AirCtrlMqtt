# AirCtrlMqtt

A Rust application that reads sensor data from TFA Dostmann AIRCO2NTROL devices and publishes the readings to an MQTT broker.

## Features

- Real-time monitoring of CO2, temperature, and humidity sensors
- MQTT integration for remote data collection
- Docker containerization for easy deployment
- Configurable via environment variables

## Sensor Data

The application publishes the following sensor readings:

- **CO2**: Parts per million (ppm)
- **Temperature**: Degrees Celsius (°C)
- **Humidity**: Percentage (%)

Data is published to a configurable MQTT topic with timestamp information.

## Configuration

Configuration is handled through environment variables:

- `MQTT_HOST`: MQTT broker hostname
- `MQTT_PORT`: MQTT broker port
- `MQTT_CLIENT_ID`: MQTT client identifier
- `MQTT_USERNAME`: MQTT authentication username
- `MQTT_PASSWORD`: MQTT authentication password
- `MQTT_TOPIC`: MQTT topic for sensor data

## Supported Devices

- [TFA Dostmann AIRCO2NTROL Mini](https://www.tfa-dostmann.de/produkt/co2-monitor-airco2ntrol-mini-31-5006/)
- [TFA Dostmann AIRCO2NTROL Coach](https://www.tfa-dostmann.de/produkt/co2-monitor-airco2ntrol-coach-31-5009/) (untested)

## Requirements

- Supported air quality monitoring device (see above)
- MQTT broker for data collection
- Docker (for containerized deployment)

## Usage

_Usage instructions will be added after Docker image publication._

## Todo

- [ ] Home Assistant entity discovery

## Credits

Thanks to [DJE98](https://github.com/DJE98) for the [aircontrol](https://crates.io/crates/aircontrol) crate!
