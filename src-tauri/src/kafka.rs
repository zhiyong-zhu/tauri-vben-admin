use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::util::Timeout;
use anyhow::Result;
use std::time::Duration;
use crate::models::DeviceSignal;
use crate::config::KafkaConfig;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub async fn new(config: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("client.id", &config.client_id)
            .set("message.timeout.ms", "5000")
            .set("queue.buffering.max.messages", "10000")
            .set("queue.buffering.max.ms", "0")
            .set("batch.num.messages", "1000")
            .set("socket.timeout.ms", "3000")  // 减少超时时间
            .set("metadata.request.timeout.ms", "3000")  // 减少元数据请求超时
            .create()?;

        // 测试连接 - 使用更短的超时时间
        match producer.client().fetch_metadata(None, Timeout::After(Duration::from_secs(3))) {
            Ok(metadata) => {
                log::info!("Kafka connection established. Brokers: {:?}", 
                          metadata.brokers().iter().map(|b| format!("{}:{}", b.host(), b.port())).collect::<Vec<_>>());
            },
            Err(e) => {
                log::warn!("Failed to connect to Kafka brokers: {}", e);
                log::warn!("Kafka functionality will be limited. Please check broker configuration.");
                // 不抛出错误，继续创建客户端
            }
        }

        Ok(Self {
            producer,
            topic: config.topic.clone(),
        })
    }

    pub async fn send_signal(&self, signal: &DeviceSignal) -> Result<()> {
        let key = format!("{}_{}", signal.device_id, signal.signal_type);
        let payload = serde_json::to_string(signal)?;

        let record = FutureRecord::to(&self.topic)
            .key(&key)
            .payload(&payload);

        let delivery_status = self.producer
            .send(record, Timeout::After(Duration::from_secs(5)))
            .await;

        match delivery_status {
            Ok((partition, offset)) => {
                log::debug!(
                    "Message sent to Kafka: topic={}, partition={}, offset={}, device_id={}", 
                    self.topic, partition, offset, signal.device_id
                );
                Ok(())
            },
            Err((kafka_error, _)) => {
                log::error!("Failed to send message to Kafka: {}", kafka_error);
                Err(kafka_error.into())
            }
        }
    }

    pub async fn send_batch_signals(&self, signals: &[DeviceSignal]) -> Result<()> {
        if signals.is_empty() {
            return Ok(());
        }

        let mut success_count = 0;
        let mut error_count = 0;

        for signal in signals {
            let key = format!("{}_{}", signal.device_id, signal.signal_type);
            let payload = serde_json::to_string(signal)?;

            let record = FutureRecord::to(&self.topic)
                .key(&key)
                .payload(&payload);

            match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
                Ok((partition, offset)) => {
                    success_count += 1;
                    log::debug!("Batch message sent: partition={}, offset={}", partition, offset);
                },
                Err((kafka_error, _)) => {
                    error_count += 1;
                    log::error!("Failed to send batch message: {}", kafka_error);
                }
            }
        }

        log::info!("Batch send completed: {} success, {} errors", success_count, error_count);

        if error_count > 0 {
            Err(anyhow::anyhow!("Failed to send {} out of {} messages", error_count, signals.len()))
        } else {
            Ok(())
        }
    }

    pub async fn send_custom_message(&self, key: &str, payload: &str) -> Result<()> {
        let record = FutureRecord::to(&self.topic)
            .key(key)
            .payload(payload);

        let delivery_status = self.producer
            .send(record, Timeout::After(Duration::from_secs(5)))
            .await;

        match delivery_status {
            Ok((partition, offset)) => {
                log::debug!("Custom message sent: partition={}, offset={}, key={}", partition, offset, key);
                Ok(())
            },
            Err((kafka_error, _)) => {
                log::error!("Failed to send custom message: {}", kafka_error);
                Err(kafka_error.into())
            }
        }
    }

    pub async fn health_check(&self) -> Result<bool> {
        // 获取主题元数据来验证连接
        match self.producer.client().fetch_metadata(
            Some(&self.topic), 
            Timeout::After(Duration::from_secs(3))
        ) {
            Ok(metadata) => {
                let topic_metadata = metadata.topics()
                    .iter()
                    .find(|t| t.name() == self.topic);

                match topic_metadata {
                    Some(topic) => {
                        if topic.partitions().is_empty() {
                            log::warn!("Kafka topic '{}' has no partitions", self.topic);
                            Ok(false)
                        } else {
                            log::debug!("Kafka health check passed. Topic '{}' has {} partitions", 
                                      self.topic, topic.partitions().len());
                            Ok(true)
                        }
                    },
                    None => {
                        log::warn!("Kafka topic '{}' not found", self.topic);
                        Ok(false)
                    }
                }
            },
            Err(e) => {
                log::warn!("Kafka health check failed: {}", e);
                Ok(false)  // 返回false而不是抛出错误
            }
        }
    }

    pub fn flush(&self, timeout: Duration) -> Result<()> {
        self.producer.flush(Timeout::After(timeout))?;
        log::debug!("Kafka producer flushed");
        Ok(())
    }
}