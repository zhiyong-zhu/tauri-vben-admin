use influxdb::{Client, InfluxDbWriteable, Timestamp};
use anyhow::Result;
use crate::models::{DeviceSignal, TimeSeriesPoint};
use crate::config::InfluxDbConfig;

#[derive(Clone)]
pub struct InfluxDbClient {
    client: Client,
    database: String,
}

impl InfluxDbClient {
    pub async fn new(config: &InfluxDbConfig) -> Result<Self> {
        let mut client = Client::new(&config.url, &config.database);
        
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            client = client.with_auth(username, password);
        }

        let influx_client = Self {
            client,
            database: config.database.clone(),
        };

        // 测试连接
        influx_client.health_check().await?;
        log::info!("InfluxDB connection established successfully");
        
        Ok(influx_client)
    }

    pub async fn write_signal(&self, signal: &DeviceSignal) -> Result<()> {
        let point = TimeSeriesPoint::from(signal);
        self.write_point(&point).await
    }

    pub async fn write_point(&self, point: &TimeSeriesPoint) -> Result<()> {
        let mut influx_point = influxdb::WriteQuery::new(
            Timestamp::Milliseconds(point.timestamp.timestamp_millis() as u128),
            &point.measurement
        );

        // 添加标签
        for (key, value) in &point.tags {
            influx_point = influx_point.add_tag(key, value.as_str());
        }

        // 添加字段
        for (key, value) in &point.fields {
            influx_point = influx_point.add_field(key, *value);
        }

        self.client.query(influx_point).await?;
        
        log::debug!("Written point to InfluxDB: {}", point.measurement);
        Ok(())
    }

    pub async fn write_batch_signals(&self, signals: &[DeviceSignal]) -> Result<()> {
        if signals.is_empty() {
            return Ok(());
        }

        let points: Vec<TimeSeriesPoint> = signals.iter().map(TimeSeriesPoint::from).collect();
        self.write_batch_points(&points).await
    }

    pub async fn write_batch_points(&self, points: &[TimeSeriesPoint]) -> Result<()> {
        if points.is_empty() {
            return Ok(());
        }

        let mut queries = Vec::new();
        
        for point in points {
            let mut influx_point = influxdb::WriteQuery::new(
                Timestamp::Milliseconds(point.timestamp.timestamp_millis() as u128),
                &point.measurement
            );

            // 添加标签
            for (key, value) in &point.tags {
                influx_point = influx_point.add_tag(key, value.as_str());
            }

            // 添加字段
            for (key, value) in &point.fields {
                influx_point = influx_point.add_field(key, *value);
            }

            queries.push(influx_point);
        }

        // 批量写入
        for query in queries {
            self.client.query(query).await?;
        }

        log::debug!("Written {} points to InfluxDB", points.len());
        Ok(())
    }

    pub async fn query_recent_signals(&self, device_id: Option<&str>, limit: Option<u32>) -> Result<Vec<DeviceSignal>> {
        // 注意: InfluxDB 1.8的查询结果解析比较复杂
        // 这里提供一个基础的框架，实际使用时可能需要根据具体的InfluxDB返回格式进行调整
        let mut signals = Vec::new();
        
        // 由于influxdb crate在查询方面的限制，这里暂时返回空集合
        // 在实际使用中，您可能需要使用HTTP客户端直接查询InfluxDB API
        log::warn!("Query functionality requires direct HTTP client implementation for InfluxDB 1.8");
        
        Ok(signals)
    }

    pub async fn health_check(&self) -> Result<bool> {
        // 尝试查询数据库信息
        let ping_result = self.client.ping().await;
        match ping_result {
            Ok(_) => {
                log::info!("InfluxDB health check passed");
                Ok(true)
            },
            Err(e) => {
                log::error!("InfluxDB health check failed: {}", e);
                Err(e.into())
            }
        }
    }

    pub async fn create_database(&self) -> Result<()> {
        // 数据库创建通常在InfluxDB服务器端或通过HTTP API完成
        // 这里只是记录信息
        log::info!("InfluxDB database '{}' should be created manually or via HTTP API", self.database);
        Ok(())
    }
}