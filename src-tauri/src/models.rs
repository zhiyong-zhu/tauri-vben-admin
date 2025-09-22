use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 设备信号数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSignal {
    pub id: Option<Uuid>,
    pub device_id: String,
    pub signal_type: String,
    pub value: f64,
    pub unit: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

/// API接收的设备信号请求
#[derive(Debug, Deserialize)]
pub struct DeviceSignalRequest {
    pub device_id: String,
    pub signal_type: String,
    pub value: f64,
    pub unit: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// API响应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
        }
    }
}

/// InfluxDB时间序列数据点
#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub measurement: String,
    pub tags: std::collections::HashMap<String, String>,
    pub fields: std::collections::HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
}

impl From<&DeviceSignal> for TimeSeriesPoint {
    fn from(signal: &DeviceSignal) -> Self {
        let mut tags = std::collections::HashMap::new();
        tags.insert("device_id".to_string(), signal.device_id.clone());
        tags.insert("signal_type".to_string(), signal.signal_type.clone());
        
        if let Some(unit) = &signal.unit {
            tags.insert("unit".to_string(), unit.clone());
        }

        let mut fields = std::collections::HashMap::new();
        fields.insert("value".to_string(), signal.value);

        Self {
            measurement: "device_signals".to_string(),
            tags,
            fields,
            timestamp: signal.timestamp,
        }
    }
}