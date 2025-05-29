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

### 1. Device Setup

First, create a udev rule to create a stable device symlink. Create the file `/etc/udev/rules.d/99-aircontrol.rules`:

```
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="04d9", ATTRS{idProduct}=="a052", SYMLINK+="aircontrol", MODE="0666"
```

Then reload udev rules:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### 2. Docker Compose

Create a `docker-compose.yml` file:

```yaml
services:
  airctrl-mqtt:
    image: cupcaker/air-ctrl-mqtt:latest
    container_name: airctrl-mqtt
    environment:
      - MQTT_HOST=your-mqtt-broker-host
      - MQTT_PORT=1883
      - MQTT_CLIENT_ID=airctrl_client
      - MQTT_TOPIC=airctrl/sensors
      - MQTT_USERNAME=your-username # Optional
      - MQTT_PASSWORD=your-password # Optional
    devices:
      - /dev/aircontrol:/dev/aircontrol
    restart: unless-stopped
    stop_grace_period: 2s
```

Update the environment variables with your MQTT broker settings, then run:

```bash
docker-compose up -d
```

### 3. Docker Run

Alternatively, run directly with Docker:

```bash
docker run -d \
  --name airctrl-mqtt \
  --device /dev/aircontrol:/dev/aircontrol \
  -e MQTT_HOST=your-mqtt-broker-host \
  -e MQTT_PORT=1883 \
  -e MQTT_CLIENT_ID=airctrl_client \
  -e MQTT_TOPIC=airctrl/sensors \
  -e MQTT_USERNAME=your-username \
  -e MQTT_PASSWORD=your-password \
  --restart unless-stopped \
  cupcaker/air-ctrl-mqtt:latest
```

## Todo

- [ ] Home Assistant entity discovery

## Credits

Thanks to [DJE98](https://github.com/DJE98) for the [aircontrol](https://crates.io/crates/aircontrol) crate!
