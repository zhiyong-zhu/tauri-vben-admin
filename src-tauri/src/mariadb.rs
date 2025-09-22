use sqlx::{MySqlPool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::DeviceSignal;
use crate::config::MariaDbConfig;

#[derive(Clone)]
pub struct MariaDbClient {
    pool: MySqlPool,
}

impl MariaDbClient {
    pub async fn new(config: &MariaDbConfig) -> Result<Self> {
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );

        let pool = MySqlPool::connect(&database_url).await?;
        
        // 创建表
        let client = Self { pool };
        client.create_tables().await?;
        
        Ok(client)
    }

    async fn create_tables(&self) -> Result<()> {
        let create_table_sql = r#"
            CREATE TABLE IF NOT EXISTS device_signals (
                id CHAR(36) PRIMARY KEY,
                device_id VARCHAR(255) NOT NULL,
                signal_type VARCHAR(255) NOT NULL,
                value DOUBLE NOT NULL,
                unit VARCHAR(50),
                timestamp DATETIME(6) NOT NULL,
                metadata JSON,
                created_at DATETIME(6) DEFAULT CURRENT_TIMESTAMP(6),
                INDEX idx_device_id (device_id),
                INDEX idx_signal_type (signal_type),
                INDEX idx_timestamp (timestamp)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4
        "#;

        sqlx::query(create_table_sql).execute(&self.pool).await?;
        log::info!("MariaDB tables created successfully");
        
        Ok(())
    }

    pub async fn insert_signal(&self, signal: &DeviceSignal) -> Result<()> {
        let id = signal.id.unwrap_or_else(|| Uuid::new_v4());
        
        let insert_sql = r#"
            INSERT INTO device_signals (id, device_id, signal_type, value, unit, timestamp, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(insert_sql)
            .bind(id.to_string())
            .bind(&signal.device_id)
            .bind(&signal.signal_type)
            .bind(signal.value)
            .bind(&signal.unit)
            .bind(signal.timestamp)
            .bind(signal.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap_or_default()))
            .execute(&self.pool)
            .await?;

        log::debug!("Inserted signal to MariaDB: {} - {}", signal.device_id, signal.signal_type);
        Ok(())
    }

    pub async fn get_signals_by_device(&self, device_id: &str, limit: Option<i32>) -> Result<Vec<DeviceSignal>> {
        let limit = limit.unwrap_or(100);
        
        let query_sql = r#"
            SELECT id, device_id, signal_type, value, unit, timestamp, metadata
            FROM device_signals
            WHERE device_id = ?
            ORDER BY timestamp DESC
            LIMIT ?
        "#;

        let rows = sqlx::query(query_sql)
            .bind(device_id)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut signals = Vec::new();
        for row in rows {
            let metadata_str: Option<String> = row.try_get("metadata")?;
            let metadata = metadata_str
                .and_then(|s| serde_json::from_str(&s).ok());

            let signal = DeviceSignal {
                id: Some(Uuid::parse_str(&row.try_get::<String, _>("id")?)?),
                device_id: row.try_get("device_id")?,
                signal_type: row.try_get("signal_type")?,
                value: row.try_get("value")?,
                unit: row.try_get("unit")?,
                timestamp: row.try_get::<DateTime<Utc>, _>("timestamp")?,
                metadata,
            };
            signals.push(signal);
        }

        Ok(signals)
    }

    pub async fn get_latest_signals(&self, limit: Option<i32>) -> Result<Vec<DeviceSignal>> {
        let limit = limit.unwrap_or(50);
        
        let query_sql = r#"
            SELECT id, device_id, signal_type, value, unit, timestamp, metadata
            FROM device_signals
            ORDER BY timestamp DESC
            LIMIT ?
        "#;

        let rows = sqlx::query(query_sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut signals = Vec::new();
        for row in rows {
            let metadata_str: Option<String> = row.try_get("metadata")?;
            let metadata = metadata_str
                .and_then(|s| serde_json::from_str(&s).ok());

            let signal = DeviceSignal {
                id: Some(Uuid::parse_str(&row.try_get::<String, _>("id")?)?),
                device_id: row.try_get("device_id")?,
                signal_type: row.try_get("signal_type")?,
                value: row.try_get("value")?,
                unit: row.try_get("unit")?,
                timestamp: row.try_get::<DateTime<Utc>, _>("timestamp")?,
                metadata,
            };
            signals.push(signal);
        }

        Ok(signals)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let result = sqlx::query("SELECT 1 as test")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.try_get::<i32, _>("test")? == 1)
    }
}