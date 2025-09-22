# 🚀 快速入门指南

## 📋 系统要求

在运行Tauri设备信号网关之前，您需要确保以下服务正在运行：

### 1. MariaDB 10.4 设置

```bash
# macOS 安装
brew install mariadb
brew services start mariadb

# 创建数据库
mysql -u root -p
CREATE DATABASE device_data;
GRANT ALL PRIVILEGES ON device_data.* TO 'root'@'localhost';
FLUSH PRIVILEGES;
EXIT;
```

### 2. InfluxDB 1.8 设置

```bash
# macOS 安装
brew install influxdb@1
brew services start influxdb@1

# 创建数据库
curl -XPOST "http://localhost:8086/query" --data-urlencode "q=CREATE DATABASE device_signals"
```

### 3. Kafka 0.9+ 设置

```bash
# macOS 安装
brew install kafka
brew services start zookeeper
brew services start kafka

# 创建主题
kafka-topics --create --topic device-signals --bootstrap-server localhost:9092 --partitions 3 --replication-factor 1
```

## 🔧 配置文件

项目提供了两个配置文件示例：

### 1. 本地开发配置

如果您在本地开发，请使用 `config.local.toml`：

```bash
# 复制本地配置
cp config.local.toml config.toml
```

### 2. 自定义配置

编辑 `config.toml` 文件以匹配您的环境：

```toml
[server]
host = "127.0.0.1"
port = 8080

[mariadb]
host = "localhost"
port = 3306
database = "device_data"
username = "root"
password = "your_password"  # 设置您的MariaDB密码

[influxdb]
url = "http://localhost:8086"
database = "device_signals"

[kafka]
brokers = "localhost:9092"
topic = "device-signals"
client_id = "tauri-device-gateway"
```

## 🚀 启动应用

```bash
# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

## 🧪 测试API

应用启动后，API将在 http://localhost:8080 运行：

```bash
# 发送设备信号
curl -X POST http://localhost:8080/api/signals \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "sensor_001",
    "signal_type": "temperature", 
    "value": 25.5,
    "unit": "°C"
  }'

# 检查健康状态
curl http://localhost:8080/api/health

# 获取最新信号
curl http://localhost:8080/api/signals/latest
```

## 🔍 故障排除

### 1. MariaDB连接失败
- 确保MariaDB服务运行：`brew services list | grep mariadb`
- 检查用户权限：`mysql -u root -p` 并测试连接
- 验证数据库存在：`SHOW DATABASES;`

### 2. InfluxDB连接失败
- 确保InfluxDB运行：`brew services list | grep influxdb`
- 测试连接：`curl http://localhost:8086/ping`
- 检查数据库：`curl "http://localhost:8086/query?q=SHOW+DATABASES"`

### 3. Kafka连接失败
- 确保Zookeeper和Kafka都在运行
- 检查主题：`kafka-topics --list --bootstrap-server localhost:9092`
- 测试连接：`kafka-console-producer --topic device-signals --bootstrap-server localhost:9092`

## 📚 更多信息

- 查看 [API_GUIDE.md](./API_GUIDE.md) 了解完整的API文档
- 查看 [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) 了解项目架构