use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataSourceConfig {
    pub r#type: String,
    pub database: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
