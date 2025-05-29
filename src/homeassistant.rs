use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct HaDiscoveryPayload {
    pub device: HaDiscoveryDevice,
    pub components: HashMap<String, HaDiscoveryComponent>,
    pub origin: HaDiscoveryOrigin,
}

#[derive(Serialize)]
pub struct HaDiscoveryOrigin {
    pub name: String,
    pub sw_version: String,
    pub support_url: String,
}

#[derive(Serialize)]
pub struct HaDiscoveryDevice {
    pub identifiers: Vec<String>,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub sw_version: String,
}

#[derive(Serialize)]
pub struct HaDiscoveryComponent {
    pub platform: String,
    pub device_class: Option<String>,
    pub name: String,
    pub state_topic: String,
    pub unit_of_measurement: Option<String>,
    pub value_template: String,
    pub unique_id: String,
    pub icon: Option<String>,
}

#[derive(Serialize)]
pub struct HaDiscoverySensor {
    pub device_class: Option<String>,
    pub name: String,
    pub state_topic: String,
    pub unit_of_measurement: String,
    pub value_template: String,
    pub unique_id: String,
    pub device: HaDiscoveryDevice,
    pub availability_topic: Option<String>,
    pub icon: Option<String>,
}

pub struct HomeAssistantDiscovery {
    pub discovery_prefix: String,
    pub object_id: String,
    pub base_topic: String,
}

impl HomeAssistantDiscovery {
    pub fn new(object_id: String, base_topic: String, discovery_prefix: String) -> Self {
        Self {
            discovery_prefix,
            object_id,
            base_topic,
        }
    }

    fn create_device_payload(&self) -> HaDiscoveryPayload {
        let mut components = HashMap::new();

        // CO2 sensor
        components.insert(
            "co2".to_string(),
            HaDiscoveryComponent {
                platform: "sensor".into(),
                device_class: Some("carbon_dioxide".into()),
                name: "CO2".into(),
                state_topic: self.base_topic.clone(),
                unit_of_measurement: Some("ppm".into()),
                value_template: "{{ value_json.co2 }}".into(),
                unique_id: format!("{}_co2", self.object_id),
                icon: Some("mdi:molecule-co2".into()),
            },
        );

        // Temperature sensor
        components.insert(
            "temperature".to_string(),
            HaDiscoveryComponent {
                platform: "sensor".into(),
                device_class: Some("temperature".into()),
                name: "Temperature".into(),
                state_topic: self.base_topic.clone(),
                unit_of_measurement: Some("°C".into()),
                value_template: "{{ value_json.temperature }}".into(),
                unique_id: format!("{}_temperature", self.object_id),
                icon: Some("mdi:thermometer".into()),
            },
        );

        // Humidity sensor
        components.insert(
            "humidity".to_string(),
            HaDiscoveryComponent {
                platform: "sensor".into(),
                device_class: Some("humidity".into()),
                name: "Humidity".into(),
                state_topic: self.base_topic.clone(),
                unit_of_measurement: Some("%".into()),
                value_template: "{{ value_json.humidity }}".into(),
                unique_id: format!("{}_humidity", self.object_id),
                icon: Some("mdi:water-percent".into()),
            },
        );

        let device = HaDiscoveryDevice {
            identifiers: vec![self.object_id.clone()],
            manufacturer: "TFA Dostmann".into(),
            model: "AIRCO2NTROL".into(),
            name: "TFA AIRCO2NTROL".into(),
            sw_version: format!("AirCtrlMqtt {}", env!("CARGO_PKG_VERSION")), // I know this is not the device's version but mine - a little branding never hurts
        };

        let origin = HaDiscoveryOrigin {
            name: "AirCtrlMqtt".into(),
            sw_version: env!("CARGO_PKG_VERSION").into(),
            support_url: "https://github.com/cupcaker/AirCtrlMqtt".into(),
        };

        HaDiscoveryPayload {
            device,
            components,
            origin,
        }
    }

    pub fn generate_device_discovery_config(&self) -> (String, String) {
        let topic = format!("{}/device/{}/config", self.discovery_prefix, self.object_id);

        let payload_struct = self.create_device_payload();
        let payload = serde_json::to_string(&payload_struct).unwrap();
        (topic, payload)
    }

    pub fn get_discovery_config(&self) -> (String, String) {
        self.generate_device_discovery_config()
    }
}
