use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub bond: BondConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BondConfig {
    pub default_length: f64,
    pub default_angle_deg: f64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bond: BondConfig::default(),
        }
    }
}

impl Default for BondConfig {
    fn default() -> Self {
        Self {
            default_length: 14.4,
            default_angle_deg: 120.0,
        }
    }
}
