#!/bin/bash

# Tauri设备信号网关API测试脚本

API_BASE="http://localhost:8080/api"

echo "🧪 开始测试Tauri设备信号网关API..."

# 等待服务启动
echo "⏳ 等待API服务启动..."
sleep 5

# 测试系统状态
echo ""
echo "📊 测试系统状态接口..."
curl -s "${API_BASE}/status" | jq '.' || echo "Status接口测试失败"

# 测试健康检查
echo ""
echo "🏥 测试健康检查接口..."
curl -s "${API_BASE}/health" | jq '.' || echo "Health检口测试失败"

# 测试发送单个信号
echo ""
echo "📡 测试发送单个设备信号..."
curl -s -X POST "${API_BASE}/signals" \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test_sensor_001",
    "signal_type": "temperature",
    "value": 25.5,
    "unit": "°C",
    "metadata": {
      "location": "test_room",
      "sensor_model": "TEST_DHT22"
    }
  }' | jq '.' || echo "单个信号发送失败"

# 测试发送批量信号
echo ""
echo "📦 测试批量发送设备信号..."
curl -s -X POST "${API_BASE}/signals/batch" \
  -H "Content-Type: application/json" \
  -d '[
    {
      "device_id": "test_sensor_002",
      "signal_type": "temperature",
      "value": 22.1,
      "unit": "°C"
    },
    {
      "device_id": "test_sensor_002",
      "signal_type": "humidity",
      "value": 58.3,
      "unit": "%"
    }
  ]' | jq '.' || echo "批量信号发送失败"

# 等待数据处理
echo ""
echo "⏳ 等待数据处理..."
sleep 2

# 测试获取最新信号
echo ""
echo "📈 测试获取最新信号..."
curl -s "${API_BASE}/signals/latest?limit=5" | jq '.' || echo "获取最新信号失败"

# 测试获取设备信号历史
echo ""
echo "📊 测试获取设备信号历史..."
curl -s "${API_BASE}/signals/device/test_sensor_001?limit=10" | jq '.' || echo "获取设备历史失败"

# 测试Kafka消息
echo ""
echo "🔔 测试Kafka消息发送..."
curl -s -X POST "${API_BASE}/test/kafka" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "test_message_key",
    "message": "Hello from API test!"
  }' | jq '.' || echo "Kafka测试消息发送失败"

echo ""
echo "✅ API测试完成！"
echo ""
echo "💡 提示："
echo "- 确保MariaDB、InfluxDB和Kafka服务正在运行"
echo "- 检查config.toml配置文件中的连接信息"
echo "- 查看应用日志了解详细信息"