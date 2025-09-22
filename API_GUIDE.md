# Tauri设备信号网关

这是一个基于Tauri的设备信号网关应用，提供HTTP API接口来接收设备信号，并将数据存储到MariaDB、InfluxDB，同时发送消息到Kafka。

## 功能特性

- 🌐 HTTP API服务器接收设备信号
- 🗄️ MariaDB 10.4 存储结构化数据
- 📊 InfluxDB 1.8 存储时间序列数据
- 📫 Kafka 0.9.0.0 消息队列
- 🖥️ Tauri桌面应用界面
- 🔍 健康检查和监控接口

## API接口

### 发送单个设备信号
```bash
POST /api/signals
Content-Type: application/json

{
  "device_id": "sensor_001",
  "signal_type": "temperature",
  "value": 25.5,
  "unit": "°C",
  "metadata": {
    "location": "room_1",
    "sensor_model": "DHT22"
  }
}
```

### 批量发送设备信号
```bash
POST /api/signals/batch
Content-Type: application/json

[
  {
    "device_id": "sensor_001",
    "signal_type": "temperature",
    "value": 25.5,
    "unit": "°C"
  },
  {
    "device_id": "sensor_001",
    "signal_type": "humidity",
    "value": 60.0,
    "unit": "%"
  }
]
```

### 获取设备信号历史
```bash
GET /api/signals/device/{device_id}?limit=100
```

### 获取最新信号
```bash
GET /api/signals/latest?limit=50
```

### 健康检查
```bash
GET /api/health
```

### 系统状态
```bash
GET /api/status
```

### 测试Kafka消息
```bash
POST /api/test/kafka
Content-Type: application/json

{
  "key": "test_key",
  "message": "test message"
}
```

## 配置文件

应用使用 `config.toml` 文件进行配置，也可以通过环境变量覆盖：

```toml
[server]
host = "127.0.0.1"
port = 8080

[mariadb]
host = "localhost"
port = 3306
database = "device_data"
username = "root"
password = ""

[influxdb]
url = "http://localhost:8086"
database = "device_signals"

[kafka]
brokers = "localhost:9092"
topic = "device-signals"
client_id = "tauri-device-gateway"
```

## 环境变量

可以使用环境变量覆盖配置：

```bash
export APP_SERVER_HOST=0.0.0.0
export APP_SERVER_PORT=9090
export APP_MARIADB_HOST=mysql-server
export APP_MARIADB_PASSWORD=secret
export APP_INFLUXDB_URL=http://influxdb:8086
export APP_KAFKA_BROKERS=kafka1:9092,kafka2:9092
```

## 数据库准备

### MariaDB
```sql
CREATE DATABASE device_data;
-- 表会自动创建
```

### InfluxDB
```bash
# InfluxDB 1.8
curl -XPOST "http://localhost:8086/query" --data-urlencode "q=CREATE DATABASE device_signals"
```

### Kafka
```bash
# 创建主题
kafka-topics.sh --create --topic device-signals --bootstrap-server localhost:9092 --partitions 3 --replication-factor 1
```

## 构建和运行

```bash
# 开发模式
pnpm tauri dev

# 构建
pnpm tauri build
```

## 示例使用

```bash
# 发送温度信号
curl -X POST http://localhost:8080/api/signals \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "temp_sensor_01",
    "signal_type": "temperature",
    "value": 23.5,
    "unit": "°C",
    "metadata": {"room": "office"}
  }'

# 检查健康状态
curl http://localhost:8080/api/health

# 获取最新信号
curl http://localhost:8080/api/signals/latest?limit=10
```

## 监控和日志

应用使用标准的Rust logging，可以通过环境变量控制日志级别：

```bash
export RUST_LOG=info
# 或者
export RUST_LOG=debug
```

## 错误处理

所有API都返回统一的JSON格式：

```json
{
  "success": true,
  "message": "Success",
  "data": {...}
}
```

错误响应：
```json
{
  "success": false,
  "message": "Error description",
  "data": null
}
```