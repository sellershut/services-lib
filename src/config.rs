use serde::Deserialize;

use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(skip)]
    pub name: Arc<str>,
    #[serde(skip)]
    pub version: Arc<str>,
    pub env: Environment,
    #[cfg(feature = "api")]
    pub port: u16,
    #[serde(default)]
    #[serde(rename = "log-level")]
    #[cfg(feature = "tracing")]
    pub log_level: Option<Arc<str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub application: AppConfig,
    #[cfg(feature = "postgres")]
    pub database: crate::postgres::PostgresConfig,
    #[serde(default)]
    pub misc: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Development => "development",
                Environment::Production => "production",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the enum is correctly serialized and deserialized
    #[test]
    fn test_environment_serialization() {
        // Test serialization for Development
        let dev = Environment::Development;
        let dev_json = serde_json::to_string(&dev).unwrap();
        assert_eq!(dev_json, "\"development\"");

        // Test serialization for Production
        let prod = Environment::Production;
        let prod_json = serde_json::to_string(&prod).unwrap();
        assert_eq!(prod_json, "\"production\"");

        // Test deserialization for Development
        let dev_str = "\"development\"";
        let deserialized_dev: Environment = serde_json::from_str(dev_str).unwrap();
        assert_eq!(deserialized_dev, Environment::Development);

        // Test deserialization for Production
        let prod_str = "\"production\"";
        let deserialized_prod: Environment = serde_json::from_str(prod_str).unwrap();
        assert_eq!(deserialized_prod, Environment::Production);
    }

    // Test Display implementation
    #[test]
    fn test_environment_display() {
        let dev = Environment::Development;
        assert_eq!(format!("{}", dev), "development");

        let prod = Environment::Production;
        assert_eq!(format!("{}", prod), "production");
    }
}
