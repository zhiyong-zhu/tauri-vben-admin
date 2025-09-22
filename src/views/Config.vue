<template>
  <div class="config-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">设备信号网关 - 配置管理</span>
        </div>
        <a-menu
          theme="dark"
          mode="horizontal"
          :default-selected-keys="['3']"
          class="menu"
        >
          <a-menu-item key="1" @click="$router.push('/')">
            <HomeOutlined />
            首页
          </a-menu-item>
          <a-menu-item key="2" @click="$router.push('/dashboard')">
            <DashboardOutlined />
            仪表板
          </a-menu-item>
          <a-menu-item key="3" @click="$router.push('/config')">
            <SettingOutlined />
            配置管理
          </a-menu-item>
          <a-menu-item key="4" @click="$router.push('/api-test')">
            <ApiOutlined />
            API测试
          </a-menu-item>
        </a-menu>
      </a-layout-header>
      
      <a-layout-content class="content">
        <div class="config-content">
          <a-page-header
            class="page-header"
            title="配置管理"
            sub-title="管理数据库和消息队列连接配置"
          />

          <!-- 连接状态概览 -->
          <a-card title="连接状态" class="status-card" style="margin-bottom: 24px;">
            <a-row :gutter="[16, 16]">
              <a-col :span="8">
                <a-card size="small" :class="['status-item', mariadbStatus ? 'status-success' : 'status-error']">
                  <template #title>
                    <DatabaseOutlined />
                    MariaDB
                  </template>
                  <a-tag :color="mariadbStatus ? 'green' : 'red'">
                    {{ mariadbStatus ? '已连接' : '连接失败' }}
                  </a-tag>
                  <div style="margin-top: 8px;">
                    <small>{{ config.mariadb.host }}:{{ config.mariadb.port }}</small>
                  </div>
                </a-card>
              </a-col>
              <a-col :span="8">
                <a-card size="small" :class="['status-item', influxdbStatus ? 'status-success' : 'status-error']">
                  <template #title>
                    <LineChartOutlined />
                    InfluxDB
                  </template>
                  <a-tag :color="influxdbStatus ? 'green' : 'red'">
                    {{ influxdbStatus ? '已连接' : '连接失败' }}
                  </a-tag>
                  <div style="margin-top: 8px;">
                    <small>{{ config.influxdb.url }}</small>
                  </div>
                </a-card>
              </a-col>
              <a-col :span="8">
                <a-card size="small" :class="['status-item', kafkaStatus ? 'status-success' : 'status-error']">
                  <template #title>
                    <CloudOutlined />
                    Kafka
                  </template>
                  <a-tag :color="kafkaStatus ? 'green' : 'red'">
                    {{ kafkaStatus ? '已连接' : '连接失败' }}
                  </a-tag>
                  <div style="margin-top: 8px;">
                    <small>{{ config.kafka.brokers }}</small>
                  </div>
                </a-card>
              </a-col>
            </a-row>
            <div style="margin-top: 16px; text-align: center;">
              <a-button type="primary" @click="checkHealth" :loading="healthChecking">
                <ReloadOutlined />
                刷新状态
              </a-button>
            </div>
          </a-card>

          <!-- 配置表单 -->
          <a-tabs v-model:activeKey="activeTab" type="card">
            <!-- 服务器配置 -->
            <a-tab-pane key="server" tab="服务器配置">
              <a-card>
                <a-form :model="config.server" layout="vertical">
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="服务器地址" name="host">
                        <a-input v-model:value="config.server.host" placeholder="127.0.0.1" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="端口" name="port">
                        <a-input-number v-model:value="config.server.port" :min="1" :max="65535" style="width: 100%" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                </a-form>
              </a-card>
            </a-tab-pane>

            <!-- MariaDB配置 -->
            <a-tab-pane key="mariadb" tab="MariaDB配置">
              <a-card>
                <a-form :model="config.mariadb" layout="vertical">
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="主机地址" name="host">
                        <a-input v-model:value="config.mariadb.host" placeholder="localhost" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="端口" name="port">
                        <a-input-number v-model:value="config.mariadb.port" :min="1" :max="65535" style="width: 100%" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-row :gutter="16">
                    <a-col :span="8">
                      <a-form-item label="数据库名" name="database">
                        <a-input v-model:value="config.mariadb.database" placeholder="device_data" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="8">
                      <a-form-item label="用户名" name="username">
                        <a-input v-model:value="config.mariadb.username" placeholder="root" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="8">
                      <a-form-item label="密码" name="password">
                        <a-input-password v-model:value="config.mariadb.password" placeholder="请输入密码" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-form-item>
                    <a-button type="primary" @click="testMariaDB" :loading="testing.mariadb">
                      <DatabaseOutlined />
                      测试连接
                    </a-button>
                  </a-form-item>
                </a-form>
              </a-card>
            </a-tab-pane>

            <!-- InfluxDB配置 -->
            <a-tab-pane key="influxdb" tab="InfluxDB配置">
              <a-card>
                <a-form :model="config.influxdb" layout="vertical">
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="服务地址" name="url">
                        <a-input v-model:value="config.influxdb.url" placeholder="http://localhost:8086" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="数据库名" name="database">
                        <a-input v-model:value="config.influxdb.database" placeholder="device_signals" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="用户名（可选）" name="username">
                        <a-input v-model:value="config.influxdb.username" placeholder="admin" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="密码（可选）" name="password">
                        <a-input-password v-model:value="config.influxdb.password" placeholder="请输入密码" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-form-item>
                    <a-button type="primary" @click="testInfluxDB" :loading="testing.influxdb">
                      <LineChartOutlined />
                      测试连接
                    </a-button>
                  </a-form-item>
                </a-form>
              </a-card>
            </a-tab-pane>

            <!-- Kafka配置 -->
            <a-tab-pane key="kafka" tab="Kafka配置">
              <a-card>
                <a-form :model="config.kafka" layout="vertical">
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="Broker地址" name="brokers">
                        <a-input v-model:value="config.kafka.brokers" placeholder="localhost:9092" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="主题名称" name="topic">
                        <a-input v-model:value="config.kafka.topic" placeholder="device-signals" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-form-item label="客户端ID" name="client_id">
                    <a-input v-model:value="config.kafka.client_id" placeholder="tauri-device-gateway" />
                  </a-form-item>
                  <a-form-item>
                    <a-button type="primary" @click="testKafka" :loading="testing.kafka">
                      <CloudOutlined />
                      测试连接
                    </a-button>
                  </a-form-item>
                </a-form>
              </a-card>
            </a-tab-pane>
          </a-tabs>

          <!-- 保存配置 -->
          <div class="save-section">
            <a-card>
              <template #title>
                <div class="config-info">
                  <h3>配置管理</h3>
                  <div v-if="configFilePath" class="config-file-path">
                    <small style="color: #666;">配置文件位置：{{ configFilePath }}</small>
                  </div>
                </div>
              </template>
              <div style="text-align: center;">
                <a-space size="large">
                  <a-button size="large" @click="loadConfig">
                    <ReloadOutlined />
                    重新加载
                  </a-button>
                  <a-button type="primary" size="large" @click="saveConfig" :loading="saving">
                    <SaveOutlined />
                    保存配置
                  </a-button>
                </a-space>
              </div>
            </a-card>
          </div>
        </div>
      </a-layout-content>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { message, Modal } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'
import {
  HomeOutlined,
  DashboardOutlined,
  SettingOutlined,
  ApiOutlined,
  DatabaseOutlined,
  LineChartOutlined,
  CloudOutlined,
  ReloadOutlined,
  SaveOutlined
} from '@ant-design/icons-vue'

// 配置数据结构
const config = ref({
  server: {
    host: '127.0.0.1',
    port: 8080
  },
  mariadb: {
    host: 'localhost',
    port: 3306,
    database: 'device_data',
    username: 'root',
    password: ''
  },
  influxdb: {
    url: 'http://localhost:8086',
    database: 'device_signals',
    username: '',
    password: ''
  },
  kafka: {
    brokers: 'localhost:9092',
    topic: 'device-signals',
    client_id: 'tauri-device-gateway'
  }
})

// 状态管理
const activeTab = ref('server')
const saving = ref(false)
const healthChecking = ref(false)
const configFilePath = ref('')
const testing = ref({
  mariadb: false,
  influxdb: false,
  kafka: false
})

// 连接状态
const mariadbStatus = ref(false)
const influxdbStatus = ref(false)
const kafkaStatus = ref(false)

// 加载配置
const loadConfig = async () => {
  try {
    const result = await invoke('get_current_config')
    config.value = result as any
    message.success('配置加载成功')
  } catch (error) {
    console.error('加载配置失败:', error)
    message.error('加载配置失败: ' + error)
  }
}

// 获取配置文件路径
const getConfigFilePath = async () => {
  try {
    const path = await invoke('get_config_file_path')
    configFilePath.value = path as string
  } catch (error) {
    console.error('获取配置文件路径失败:', error)
  }
}

// 保存配置
const saveConfig = async () => {
  saving.value = true
  try {
    const result = await invoke('save_config', { config: config.value })
    message.success(result as string)
    
    // 更新配置文件路径显示
    await getConfigFilePath()
    
    // 提示用户是否重新加载服务
    const shouldReload = await new Promise((resolve) => {
      Modal.confirm({
        title: '配置已保存',
        content: `配置已保存到外部文件：${configFilePath.value}\n\n是否重新加载服务以应用新配置？（建议重启应用以完全应用更改）`,
        okText: '重新加载',
        cancelText: '稍后手动重启',
        onOk: () => resolve(true),
        onCancel: () => resolve(false)
      })
    })
    
    if (shouldReload) {
      try {
        const result = await invoke('reload_services', { config: config.value })
        message.info(result as string)
      } catch (error) {
        message.warning('服务重载失败，建议重启应用: ' + error)
      }
    }
  } catch (error) {
    console.error('保存配置失败:', error)
    message.error('保存配置失败: ' + error)
  } finally {
    saving.value = false
  }
}

// 健康检查
const checkHealth = async () => {
  healthChecking.value = true
  try {
    const results = await Promise.allSettled([
      invoke('test_connection', { service: 'mariadb' }),
      invoke('test_connection', { service: 'influxdb' }),
      invoke('test_connection', { service: 'kafka' })
    ])
    
    mariadbStatus.value = results[0].status === 'fulfilled' ? results[0].value as boolean : false
    influxdbStatus.value = results[1].status === 'fulfilled' ? results[1].value as boolean : false
    kafkaStatus.value = results[2].status === 'fulfilled' ? results[2].value as boolean : false
    
    message.success('状态检查完成')
  } catch (error) {
    console.error('健康检查失败:', error)
    message.error('健康检查失败')
  } finally {
    healthChecking.value = false
  }
}

// 测试MariaDB连接
const testMariaDB = async () => {
  testing.value.mariadb = true
  try {
    const result = await invoke('test_connection', { service: 'mariadb' })
    mariadbStatus.value = result as boolean
    message.success(mariadbStatus.value ? 'MariaDB连接成功' : 'MariaDB连接失败')
  } catch (error) {
    console.error('MariaDB连接测试失败:', error)
    message.error('MariaDB连接测试失败: ' + error)
    mariadbStatus.value = false
  } finally {
    testing.value.mariadb = false
  }
}

// 测试InfluxDB连接
const testInfluxDB = async () => {
  testing.value.influxdb = true
  try {
    const result = await invoke('test_connection', { service: 'influxdb' })
    influxdbStatus.value = result as boolean
    message.success(influxdbStatus.value ? 'InfluxDB连接成功' : 'InfluxDB连接失败')
  } catch (error) {
    console.error('InfluxDB连接测试失败:', error)
    message.error('InfluxDB连接测试失败: ' + error)
    influxdbStatus.value = false
  } finally {
    testing.value.influxdb = false
  }
}

// 测试Kafka连接
const testKafka = async () => {
  testing.value.kafka = true
  try {
    const result = await invoke('test_connection', { service: 'kafka' })
    kafkaStatus.value = result as boolean
    message.success(kafkaStatus.value ? 'Kafka连接成功' : 'Kafka连接失败')
  } catch (error) {
    console.error('Kafka连接测试失败:', error)
    message.error('Kafka连接测试失败: ' + error)
    kafkaStatus.value = false
  } finally {
    testing.value.kafka = false
  }
}

onMounted(() => {
  loadConfig()
  getConfigFilePath()
  checkHealth()
})
</script>

<style scoped>
.config-container {
  height: 100vh;
}

.layout {
  height: 100%;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
}

.logo {
  display: flex;
  align-items: center;
  color: white;
}

.logo-img {
  height: 32px;
  margin-right: 12px;
}

.logo-text {
  font-size: 18px;
  font-weight: bold;
}

.menu {
  flex: 1;
  justify-content: flex-end;
}

.content {
  padding: 24px;
  background: #f0f2f5;
  overflow-y: auto;
}

.config-content {
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  background: white;
  margin-bottom: 24px;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.status-card {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.status-item {
  text-align: center;
  transition: all 0.3s;
}

.status-success {
  border-color: #52c41a;
}

.status-error {
  border-color: #ff4d4f;
}

.save-section {
  margin-top: 24px;
}

.config-info h3 {
  margin-bottom: 8px;
}

.config-file-path {
  margin-top: 4px;
  padding: 4px 8px;
  background: #f6f6f6;
  border-radius: 4px;
  border: 1px solid #e8e8e8;
  word-break: break-all;
}

:deep(.ant-tabs-card > .ant-tabs-content) {
  margin-top: -16px;
}

:deep(.ant-tabs-card > .ant-tabs-content > .ant-tabs-tabpane) {
  padding: 16px;
  background: #fff;
}

:deep(.ant-tabs-card > .ant-tabs-nav::before) {
  border: none;
}

:deep(.ant-tabs-card .ant-tabs-tab) {
  border-color: transparent;
  background: transparent;
}

:deep(.ant-tabs-card .ant-tabs-tab-active) {
  border-color: #fff;
  background: #fff;
}
</style>