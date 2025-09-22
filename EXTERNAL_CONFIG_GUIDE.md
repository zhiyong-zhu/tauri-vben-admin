# 外部配置文件管理指南

## 🎯 概述

Tauri设备信号网关现已支持外部配置文件管理，配置信息会自动保存到用户配置目录，修改后持久保存，不会因应用重启而丢失。

## 📁 配置文件位置

### 系统标准配置目录

- **macOS**: `~/Library/Application Support/tauri-device-gateway/config.toml`
- **Windows**: `%APPDATA%\tauri-device-gateway\config.toml`
- **Linux**: `~/.config/tauri-device-gateway/config.toml`

### 特点

- ✅ 跨用户独立配置
- ✅ 系统标准位置，符合各平台规范
- ✅ 自动创建配置目录
- ✅ 配置持久化保存

## 🌟 新增功能

### 1. 自动配置文件管理
- 首次启动时自动创建配置文件
- 使用默认配置初始化
- 后续启动优先读取外部配置文件

### 2. 配置保存机制
- UI修改配置后自动保存到外部文件
- 保存前自动创建备份文件 (`.backup` 后缀)
- 保存路径在UI中实时显示

### 3. 智能配置加载
```rust
// 配置加载优先级
1. 外部配置文件 (用户配置目录)
2. 环境变量覆盖 (APP_ 前缀)  
3. 当前目录config.toml (兼容性)
4. 默认配置值
```

### 4. 错误容错处理
- 配置文件读取失败时使用默认值
- 保存失败时显示详细错误信息
- 配置目录创建失败时自动处理

## 🔧 使用方法

### 在UI中管理配置

1. **查看配置文件位置**
   - 打开"配置管理"页面
   - 在"配置管理"卡片标题下方可以看到当前配置文件的完整路径

2. **修改和保存配置**
   - 在各个标签页中修改配置项
   - 点击"保存配置"按钮
   - 系统会提示配置保存成功，并显示保存位置

3. **配置生效方式**
   - 配置保存后会提示是否重新加载服务
   - 建议重启应用以完全应用更改

### 手动管理配置文件

```bash
# 查看配置文件位置 (macOS)
ls -la ~/Library/Application\ Support/tauri-device-gateway/

# 查看配置内容
cat ~/Library/Application\ Support/tauri-device-gateway/config.toml

# 备份配置
cp ~/Library/Application\ Support/tauri-device-gateway/config.toml ~/config-backup.toml
```

## 📋 配置文件格式

外部配置文件使用TOML格式，结构清晰易读：

```toml
# 服务器配置
[server]
host = "127.0.0.1"
port = 8080

# MariaDB 10.4 配置
[mariadb]
host = "localhost"
port = 3306
database = "ps_v2"
username = "pike"
password = "pike"

# InfluxDB 1.8 配置  
[influxdb]
url = "http://192.168.5.199:8086"
database = "device_signals"
# username = "admin"  # 可选
# password = "password"  # 可选

# Kafka 配置
[kafka]
brokers = "192.168.5.199:9092"
topic = "device-signals"
client_id = "tauri-device-gateway"
```

## 🛡️ 安全特性

### 1. 配置备份
- 每次保存前自动创建 `.backup` 后缀的备份文件
- 避免配置丢失或损坏

### 2. 权限管理
- 配置文件存储在用户专有目录
- 遵循系统权限管理规范

### 3. 配置验证
- 保存前进行格式验证
- 连接测试确保配置正确性

## 🔄 迁移说明

### 从旧版本升级

如果你之前使用的是项目根目录的 `config.toml`：

1. **自动迁移** (首次启动)
   - 应用会检测现有的 `config.toml`
   - 自动复制到用户配置目录
   - 保持原有配置不变

2. **手动迁移**
   ```bash
   # 复制旧配置到新位置 (macOS示例)
   cp config.toml ~/Library/Application\ Support/tauri-device-gateway/
   ```

3. **环境变量继续有效**
   - 所有 `APP_` 前缀的环境变量依然可以覆盖配置
   - 提供了最高的配置优先级

## 🐛 故障排除

### 常见问题

1. **配置文件无法创建**
   ```
   错误: Failed to save configuration: Permission denied
   解决: 检查用户配置目录的写入权限
   ```

2. **配置文件读取失败**
   ```
   错误: Failed to load config: Parse error
   解决: 检查TOML格式，或删除配置文件让应用重新创建
   ```

3. **配置不生效**
   ```
   解决: 重启应用以完全应用配置更改
   ```

### 重置配置

如需重置到默认配置：

```bash
# 删除配置文件 (macOS)
rm ~/Library/Application\ Support/tauri-device-gateway/config.toml

# 重启应用，会自动创建默认配置
```

## 💡 最佳实践

1. **定期备份重要配置**
   - 系统会自动创建 `.backup` 文件
   - 建议额外备份到其他位置

2. **使用环境变量存储敏感信息**
   ```bash
   export APP_MARIADB_PASSWORD=your_secret_password
   export APP_KAFKA_BROKERS=production_broker:9092
   ```

3. **测试配置后再保存**
   - 在保存前使用"测试连接"功能
   - 确保配置正确性

4. **版本控制排除**
   - 不要将包含敏感信息的配置文件提交到版本控制
   - 使用模板文件替代

## 🔮 未来规划

- **配置同步**: 支持配置在多设备间同步
- **配置模板**: 预设的环境配置模板  
- **配置加密**: 敏感信息的加密存储
- **配置历史**: 配置版本管理和回滚功能
- **热重载**: 无需重启的配置更新机制

通过外部配置文件管理，用户可以更方便地管理和维护应用配置，确保配置的持久化和安全性。