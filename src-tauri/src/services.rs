use anyhow::Result;
use crate::models::DeviceSignal;
use crate::mariadb::MariaDbClient;
use crate::influxdb::InfluxDbClient;
use crate::kafka::KafkaProducer;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct DataService {
    mariadb: MariaDbClient,
    influxdb: InfluxDbClient,
    kafka: KafkaProducer,
}

impl DataService {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        log::info!("Initializing data service...");

        // 初始化MariaDB连接
        let mariadb = match MariaDbClient::new(&config.mariadb).await {
            Ok(client) => {
                log::info!("MariaDB client initialized");
                client
            },
            Err(e) => {
                log::error!("Failed to initialize MariaDB: {}", e);
                return Err(e);
            }
        };

        // 初始化InfluxDB连接
        let influxdb = match InfluxDbClient::new(&config.influxdb).await {
            Ok(client) => {
                log::info!("InfluxDB client initialized");
                client
            },
            Err(e) => {
                log::error!("Failed to initialize InfluxDB: {}", e);
                return Err(e);
            }
        };

        // 初始化Kafka生产者 - 允许失败
        let kafka = match KafkaProducer::new(&config.kafka).await {
            Ok(producer) => {
                log::info!("Kafka producer initialized");
                producer
            },
            Err(e) => {
                log::warn!("Failed to initialize Kafka producer: {}", e);
                log::warn!("Continuing without Kafka functionality");
                return Err(e);  // 暂时仍然返回错误，但消息更友好
            }
        };

        Ok(Self {
            mariadb,
            influxdb,
            kafka,
        })
    }

    /// 处理单个设备信号
    pub async fn process_signal(&self, signal: &DeviceSignal) -> Result<()> {
        log::debug!("Processing signal: {} - {}", signal.device_id, signal.signal_type);

        // 并行执行所有操作
        let maria_future = self.mariadb.insert_signal(signal);
        let influx_future = self.influxdb.write_signal(signal);
        let kafka_future = self.kafka.send_signal(signal);

        // 等待所有操作完成
        let (maria_result, influx_result, kafka_result) = 
            tokio::join!(maria_future, influx_future, kafka_future);

        // 处理结果
        let mut errors = Vec::new();

        if let Err(e) = maria_result {
            errors.push(format!("MariaDB error: {}", e));
        }

        if let Err(e) = influx_result {
            errors.push(format!("InfluxDB error: {}", e));
        }

        if let Err(e) = kafka_result {
            errors.push(format!("Kafka error: {}", e));
        }

        if !errors.is_empty() {
            log::error!("Signal processing errors: {:?}", errors);
            return Err(anyhow::anyhow!("Processing errors: {}", errors.join(", ")));
        }

        log::debug!("Signal processed successfully");
        Ok(())
    }

    /// 批量处理设备信号
    pub async fn process_batch_signals(&self, signals: &[DeviceSignal]) -> Result<()> {
        if signals.is_empty() {
            return Ok(());
        }

        log::debug!("Processing batch of {} signals", signals.len());

        // 并行执行所有批量操作
        let mut maria_futures = Vec::new();
        for signal in signals {
            maria_futures.push(self.mariadb.insert_signal(signal));
        }

        let influx_future = self.influxdb.write_batch_signals(signals);
        let kafka_future = self.kafka.send_batch_signals(signals);

        // 等待所有操作完成
        let influx_result = influx_future.await;
        let kafka_result = kafka_future.await;

        // 处理MariaDB结果
        let mut maria_errors = 0;
        for future in maria_futures {
            if let Err(e) = future.await {
                maria_errors += 1;
                log::error!("MariaDB batch insert error: {}", e);
            }
        }

        // 收集错误
        let mut errors = Vec::new();

        if maria_errors > 0 {
            errors.push(format!("MariaDB errors: {}/{} failed", maria_errors, signals.len()));
        }

        if let Err(e) = influx_result {
            errors.push(format!("InfluxDB error: {}", e));
        }

        if let Err(e) = kafka_result {
            errors.push(format!("Kafka error: {}", e));
        }

        if !errors.is_empty() {
            log::error!("Batch processing errors: {:?}", errors);
            return Err(anyhow::anyhow!("Batch processing errors: {}", errors.join(", ")));
        }

        log::debug!("Batch signals processed successfully");
        Ok(())
    }

    /// 获取设备信号历史
    pub async fn get_device_signals(&self, device_id: &str, limit: Option<i32>) -> Result<Vec<DeviceSignal>> {
        self.mariadb.get_signals_by_device(device_id, limit).await
    }

    /// 获取最新信号
    pub async fn get_latest_signals(&self, limit: Option<i32>) -> Result<Vec<DeviceSignal>> {
        self.mariadb.get_latest_signals(limit).await
    }

    /// MariaDB健康检查
    pub async fn mariadb_health_check(&self) -> Result<bool> {
        self.mariadb.health_check().await
    }

    /// InfluxDB健康检查
    pub async fn influxdb_health_check(&self) -> Result<bool> {
        self.influxdb.health_check().await
    }

    /// Kafka健康检查
    pub async fn kafka_health_check(&self) -> Result<bool> {
        self.kafka.health_check().await
    }

    /// 发送测试消息到Kafka
    pub async fn send_test_message(&self, key: &str, message: &str) -> Result<()> {
        self.kafka.send_custom_message(key, message).await
    }

    /// 完整的健康检查
    pub async fn full_health_check(&self) -> Result<()> {
        log::info!("Performing full health check...");

        let maria_check = self.mariadb_health_check().await;
        let influx_check = self.influxdb_health_check().await;
        let kafka_check = self.kafka_health_check().await;

        let mut errors = Vec::new();

        match maria_check {
            Ok(true) => log::info!("✓ MariaDB connection healthy"),
            Ok(false) => {
                errors.push("MariaDB connection unhealthy".to_string());
                log::error!("✗ MariaDB connection unhealthy");
            },
            Err(e) => {
                errors.push(format!("MariaDB error: {}", e));
                log::error!("✗ MariaDB error: {}", e);
            }
        }

        match influx_check {
            Ok(true) => log::info!("✓ InfluxDB connection healthy"),
            Ok(false) => {
                errors.push("InfluxDB connection unhealthy".to_string());
                log::error!("✗ InfluxDB connection unhealthy");
            },
            Err(e) => {
                errors.push(format!("InfluxDB error: {}", e));
                log::error!("✗ InfluxDB error: {}", e);
            }
        }

        match kafka_check {
            Ok(true) => log::info!("✓ Kafka connection healthy"),
            Ok(false) => {
                errors.push("Kafka connection unhealthy".to_string());
                log::error!("✗ Kafka connection unhealthy");
            },
            Err(e) => {
                errors.push(format!("Kafka error: {}", e));
                log::error!("✗ Kafka error: {}", e);
            }
        }

        if !errors.is_empty() {
            return Err(anyhow::anyhow!("Health check failed: {}", errors.join(", ")));
        }

        log::info!("✓ All systems healthy");
        Ok(())
    }
}