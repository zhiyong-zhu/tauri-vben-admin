use std::sync::Arc;
use tokio::net::TcpListener;
use anyhow::Result;

// 模块声明
mod models;
mod config;
mod mariadb;
mod influxdb;
mod kafka;
mod services;
mod api;

use config::AppConfig;
use services::DataService;
use api::{create_router, AppState};

// Tauri commands
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_api_status(state: tauri::State<'_, AppState>) -> Result<String, String> {
    match state.full_health_check().await {
        Ok(_) => Ok("All systems operational".to_string()),
        Err(e) => Err(format!("System error: {}", e)),
    }
}

#[tauri::command]
async fn get_latest_device_signals(state: tauri::State<'_, AppState>, limit: Option<i32>) -> Result<Vec<models::DeviceSignal>, String> {
    match state.get_latest_signals(limit).await {
        Ok(signals) => Ok(signals),
        Err(e) => Err(format!("Failed to get signals: {}", e)),
    }
}

#[tauri::command]
async fn test_connection(state: tauri::State<'_, AppState>, service: String) -> Result<bool, String> {
    match service.as_str() {
        "mariadb" => {
            match state.mariadb_health_check().await {
                Ok(healthy) => Ok(healthy),
                Err(e) => Err(format!("MariaDB test failed: {}", e)),
            }
        },
        "influxdb" => {
            match state.influxdb_health_check().await {
                Ok(healthy) => Ok(healthy),
                Err(e) => Err(format!("InfluxDB test failed: {}", e)),
            }
        },
        "kafka" => {
            match state.kafka_health_check().await {
                Ok(healthy) => Ok(healthy),
                Err(e) => Err(format!("Kafka test failed: {}", e)),
            }
        },
        _ => Err("Unknown service".to_string())
    }
}

#[tauri::command]
async fn send_test_signal(state: tauri::State<'_, AppState>, signal: models::DeviceSignalRequest) -> Result<String, String> {
    let device_signal = models::DeviceSignal {
        id: Some(uuid::Uuid::new_v4()),
        device_id: signal.device_id,
        signal_type: signal.signal_type,
        value: signal.value,
        unit: signal.unit,
        timestamp: chrono::Utc::now(),
        metadata: signal.metadata,
    };
    
    match state.process_signal(&device_signal).await {
        Ok(_) => Ok("Signal sent successfully".to_string()),
        Err(e) => Err(format!("Failed to send signal: {}", e)),
    }
}

#[tauri::command]
async fn get_current_config() -> Result<config::AppConfig, String> {
    match config::AppConfig::new() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Failed to load config: {}", e)),
    }
}

#[tauri::command]
async fn get_config_file_path() -> Result<String, String> {
    let config_path = config::AppConfig::get_config_path();
    Ok(config_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn save_config(config: config::AppConfig) -> Result<String, String> {
    let config_path = config::AppConfig::get_config_path();
    
    match config.save_to_file(&config_path) {
        Ok(_) => {
            log::info!("✅ Configuration saved to: {}", config_path.display());
            Ok(format!("Configuration saved successfully to: {}", config_path.display()))
        },
        Err(e) => {
            log::error!("❌ Failed to save configuration: {}", e);
            Err(format!("Failed to save configuration: {}", e))
        }
    }
}

#[tauri::command]
async fn reload_services(state: tauri::State<'_, AppState>, config: config::AppConfig) -> Result<String, String> {
    // 注意：这里我们不能直接替换整个DataService，因为Tauri的State是不可变的
    // 实际实现中，我们需要重新设计架构来支持动态重连
    // 目前先返回一个提示，建议重启应用
    Ok("Configuration updated. Please restart the application to apply changes.".to_string())
}

/// 启动HTTP API服务器
async fn start_api_server(data_service: Arc<DataService>, config: &AppConfig) -> Result<()> {
    let app = create_router(data_service.clone());
    
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    
    log::info!("🚀 API Server starting on http://{}", addr);
    log::info!("📡 Ready to receive device signals at http://{}/api/signals", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("🔧 Starting Tauri Device Gateway Application...");

    // 创建Tokio运行时来处理异步操作
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    // 在运行时中初始化服务
    let data_service = rt.block_on(async {
        log::info!("📋 Loading configuration...");
        let config = AppConfig::new().expect("Failed to load configuration");
        
        log::info!("🔌 Initializing data services...");
        let data_service = match DataService::new(&config).await {
            Ok(service) => {
                log::info!("✅ Data services initialized successfully");
                service
            },
            Err(e) => {
                log::error!("❌ Failed to initialize data services: {}", e);
                log::warn!("   📝 Please check your configuration in config.toml:");
                log::warn!("   - MariaDB: Ensure server is running and credentials are correct");
                log::warn!("   - InfluxDB: Ensure server is running on http://localhost:8086");
                log::warn!("   - Kafka: Ensure server is running on localhost:9092");
                log::warn!("   🚀 Application will start but external services may not work");
                
                // 返回错误而不是直接退出，让用户可以查看界面
                std::process::exit(1);
            }
        };
        
        log::info!("🏥 Performing initial health check...");
        if let Err(e) = data_service.full_health_check().await {
            log::warn!("⚠️  Initial health check failed: {}", e);
            log::warn!("   Application will continue, but some services may not be available.");
            log::warn!("   Please check your database and Kafka configurations in config.toml");
        }
        
        let data_service_arc = Arc::new(data_service);
        
        // 启动API服务器（在后台运行）
        let server_data_service = data_service_arc.clone();
        let server_config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = start_api_server(server_data_service, &server_config).await {
                log::error!("❌ API Server error: {}", e);
            }
        });
        
        data_service_arc
    });

    log::info!("🖥️  Starting Tauri GUI...");

    // 启动Tauri应用
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(data_service)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_api_status,
            get_latest_device_signals,
            test_connection,
            send_test_signal,
            get_current_config,
            get_config_file_path,
            save_config,
            reload_services
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
