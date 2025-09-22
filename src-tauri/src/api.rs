use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;

use crate::models::{DeviceSignal, DeviceSignalRequest, ApiResponse};
use crate::services::DataService;

pub type AppState = Arc<DataService>;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 设备信号相关接口
        .route("/api/signals", post(receive_signal))
        .route("/api/signals/batch", post(receive_batch_signals))
        .route("/api/signals/device/:device_id", get(get_device_signals))
        .route("/api/signals/latest", get(get_latest_signals))
        
        // 健康检查和状态接口
        .route("/api/health", get(health_check))
        .route("/api/status", get(system_status))
        
        // 测试接口
        .route("/api/test/kafka", post(test_kafka))
        
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // 允许跨域访问
        )
        .with_state(state)
}

/// 接收单个设备信号
async fn receive_signal(
    State(data_service): State<AppState>,
    Json(request): Json<DeviceSignalRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    log::info!("Received signal from device: {} - {}", request.device_id, request.signal_type);

    let signal = DeviceSignal {
        id: Some(Uuid::new_v4()),
        device_id: request.device_id,
        signal_type: request.signal_type,
        value: request.value,
        unit: request.unit,
        timestamp: Utc::now(),
        metadata: request.metadata,
    };

    match data_service.process_signal(&signal).await {
        Ok(_) => {
            log::debug!("Signal processed successfully");
            Ok(Json(ApiResponse::success("Signal processed successfully".to_string())))
        },
        Err(e) => {
            log::error!("Failed to process signal: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 批量接收设备信号
async fn receive_batch_signals(
    State(data_service): State<AppState>,
    Json(requests): Json<Vec<DeviceSignalRequest>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    log::info!("Received batch of {} signals", requests.len());

    let signals: Vec<DeviceSignal> = requests
        .into_iter()
        .map(|req| DeviceSignal {
            id: Some(Uuid::new_v4()),
            device_id: req.device_id,
            signal_type: req.signal_type,
            value: req.value,
            unit: req.unit,
            timestamp: Utc::now(),
            metadata: req.metadata,
        })
        .collect();

    match data_service.process_batch_signals(&signals).await {
        Ok(_) => {
            log::debug!("Batch signals processed successfully");
            Ok(Json(ApiResponse::success(format!("Processed {} signals successfully", signals.len()))))
        },
        Err(e) => {
            log::error!("Failed to process batch signals: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取指定设备的信号历史
async fn get_device_signals(
    State(data_service): State<AppState>,
    Path(device_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<DeviceSignal>>>, StatusCode> {
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(100);

    match data_service.get_device_signals(&device_id, Some(limit)).await {
        Ok(signals) => {
            log::debug!("Retrieved {} signals for device {}", signals.len(), device_id);
            Ok(Json(ApiResponse::success(signals)))
        },
        Err(e) => {
            log::error!("Failed to get device signals: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取最新的信号数据
async fn get_latest_signals(
    State(data_service): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<DeviceSignal>>>, StatusCode> {
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(50);

    match data_service.get_latest_signals(Some(limit)).await {
        Ok(signals) => {
            log::debug!("Retrieved {} latest signals", signals.len());
            Ok(Json(ApiResponse::success(signals)))
        },
        Err(e) => {
            log::error!("Failed to get latest signals: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 健康检查接口
async fn health_check(
    State(data_service): State<AppState>,
) -> Result<Json<ApiResponse<HashMap<String, bool>>>, StatusCode> {
    let mut status = HashMap::new();
    
    // 检查MariaDB连接
    match data_service.mariadb_health_check().await {
        Ok(healthy) => status.insert("mariadb".to_string(), healthy),
        Err(_) => status.insert("mariadb".to_string(), false),
    };

    // 检查InfluxDB连接
    match data_service.influxdb_health_check().await {
        Ok(healthy) => status.insert("influxdb".to_string(), healthy),
        Err(_) => status.insert("influxdb".to_string(), false),
    };

    // 检查Kafka连接
    match data_service.kafka_health_check().await {
        Ok(healthy) => status.insert("kafka".to_string(), healthy),
        Err(_) => status.insert("kafka".to_string(), false),
    };

    let all_healthy = status.values().all(|&h| h);
    
    if all_healthy {
        Ok(Json(ApiResponse::success(status)))
    } else {
        log::warn!("Health check failed: {:?}", status);
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// 系统状态接口
async fn system_status() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("service".to_string(), "Device Signal Gateway".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    status.insert("uptime".to_string(), format!("{:?}", std::time::SystemTime::now()));
    
    Json(ApiResponse::success(status))
}

/// 测试Kafka消息发送
async fn test_kafka(
    State(data_service): State<AppState>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let key = payload.get("key").unwrap_or(&"test".to_string()).clone();
    let message = payload.get("message").unwrap_or(&"test message".to_string()).clone();

    match data_service.send_test_message(&key, &message).await {
        Ok(_) => {
            log::debug!("Test message sent to Kafka");
            Ok(Json(ApiResponse::success("Test message sent successfully".to_string())))
        },
        Err(e) => {
            log::error!("Failed to send test message: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}