use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};
use std::path::PathBuf;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mariadb: MariaDbConfig,
    pub influxdb: InfluxDbConfig,
    pub kafka: KafkaConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MariaDbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InfluxDbConfig {
    pub url: String,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
    pub client_id: String,
}

impl AppConfig {
    /// 获取配置文件路径（用户配置目录）
    pub fn get_config_path() -> PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.push("tauri-device-gateway");
        
        // 确保配置目录存在
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        
        config_dir.push("config.toml");
        config_dir
    }
    
    /// 从外部配置文件加载配置
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            // 默认配置
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("mariadb.host", "localhost")?
            .set_default("mariadb.port", 3306)?
            .set_default("mariadb.database", "ps_v2")?
            .set_default("mariadb.username", "pike")?
            .set_default("mariadb.password", "pike")?
            .set_default("influxdb.url", "http://192.168.5.199:8086")?
            .set_default("influxdb.database", "device_signals")?
            .set_default("kafka.brokers", "192.168.5.199:9092")?
            .set_default("kafka.topic", "device-signals")?
            .set_default("kafka.client_id", "tauri-device-gateway")?;

        // 从指定文件加载
        if path.as_ref().exists() {
            let path_str = path.as_ref().to_str().ok_or_else(|| {
                ConfigError::Message("Invalid path".to_string())
            })?;
            config = config.add_source(File::with_name(path_str.trim_end_matches(".toml")));
        }

        // 环境变量覆盖
        config = config.add_source(Environment::with_prefix("APP"));

        config.build()?.try_deserialize()
    }
    
    /// 保存配置到外部文件
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        
        // 确保父目录存在
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 创建备份文件
        if path.as_ref().exists() {
            let backup_path = format!("{}.backup", path.as_ref().display());
            let _ = fs::copy(&path, &backup_path);
        }
        
        // 写入新配置
        let mut file = fs::File::create(&path)?;
        file.write_all(toml_string.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }

    pub fn new() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path();
        
        // 如果外部配置文件存在，优先从外部文件加载
        if config_path.exists() {
            return Self::from_file(&config_path);
        }
        
        // 否则使用默认配置并保存到外部文件
        let mut config = Config::builder()
            // 默认配置
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("mariadb.host", "localhost")?
            .set_default("mariadb.port", 3306)?
            .set_default("mariadb.database", "ps_v2")?
            .set_default("mariadb.username", "pike")?
            .set_default("mariadb.password", "pike")?
            .set_default("influxdb.url", "http://192.168.5.199:8086")?
            .set_default("influxdb.database", "device_signals")?
            .set_default("kafka.brokers", "192.168.5.199:9092")?
            .set_default("kafka.topic", "device-signals")?
            .set_default("kafka.client_id", "tauri-device-gateway")?;

        // 尝试从当前目录的 config.toml 加载（兼容性）
        if let Ok(_) = std::fs::metadata("config.toml") {
            config = config.add_source(File::with_name("config"));
        }

        // 环境变量覆盖
        config = config.add_source(Environment::with_prefix("APP"));

        let app_config: AppConfig = config.build()?.try_deserialize()?;
        
        // 保存到外部配置文件
        if let Err(e) = app_config.save_to_file(&config_path) {
            eprintln!("Warning: Failed to save config to file: {}", e);
        }
        
        Ok(app_config)
    }

    pub fn mariadb_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mariadb.username,
            self.mariadb.password,
            self.mariadb.host,
            self.mariadb.port,
            self.mariadb.database
        )
    }
}