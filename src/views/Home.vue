<template>
  <div class="home-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">设备信号网关</span>
        </div>
        <a-menu
          theme="dark"
          mode="horizontal"
          :default-selected-keys="['1']"
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
        <div class="welcome-section">
          <a-card class="welcome-card">
            <template #title>
              <span class="card-title">
                <RocketOutlined />
                欢迎使用设备信号网关
              </span>
            </template>
            
            <div class="intro-content">
              <p class="intro-text">
                一个基于 <strong>Tauri</strong> 构建的高性能设备信号采集与处理系统，支持多种数据存储和消息队列集成。
              </p>
              
              <a-row :gutter="[16, 16]" class="feature-grid">
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <DatabaseOutlined style="color: #1890ff;" />
                      MariaDB
                    </template>
                    <p>高性能关系型数据库，存储设备信息和配置数据</p>
                  </a-card>
                </a-col>
                
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <LineChartOutlined style="color: #52c41a;" />
                      InfluxDB
                    </template>
                    <p>时间序列数据库，专业存储和分析设备信号数据</p>
                  </a-card>
                </a-col>
                
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <CloudOutlined style="color: #722ed1;" />
                      Kafka
                    </template>
                    <p>高吞吐量消息队列，实时发布和处理设备信号</p>
                  </a-card>
                </a-col>
              </a-row>
              
              <div class="action-section">
                <a-space size="large">
                  <a-button type="primary" size="large" @click="$router.push('/dashboard')">
                    <DashboardOutlined />
                    查看仪表盘
                  </a-button>
                  <a-button size="large" @click="$router.push('/config')">
                    <SettingOutlined />
                    配置管理
                  </a-button>
                  <a-button size="large" @click="$router.push('/api-test')">
                    <ApiOutlined />
                    API测试
                  </a-button>
                </a-space>
              </div>
              
              <a-divider />
              
              <div class="system-status">
                <a-card size="small" title="系统状态" class="status-card">
                  <a-row :gutter="[16, 16]">
                    <a-col :span="8">
                      <div class="status-item">
                        <DatabaseOutlined style="color: #1890ff; font-size: 20px;" />
                        <div class="status-info">
                          <div class="status-title">MariaDB</div>
                          <a-badge :status="systemStatus.mariadb ? 'success' : 'error'" :text="systemStatus.mariadb ? '已连接' : '未连接'" />
                        </div>
                      </div>
                    </a-col>
                    <a-col :span="8">
                      <div class="status-item">
                        <LineChartOutlined style="color: #52c41a; font-size: 20px;" />
                        <div class="status-info">
                          <div class="status-title">InfluxDB</div>
                          <a-badge :status="systemStatus.influxdb ? 'success' : 'error'" :text="systemStatus.influxdb ? '已连接' : '未连接'" />
                        </div>
                      </div>
                    </a-col>
                    <a-col :span="8">
                      <div class="status-item">
                        <CloudOutlined style="color: #722ed1; font-size: 20px;" />
                        <div class="status-info">
                          <div class="status-title">Kafka</div>
                          <a-badge :status="systemStatus.kafka ? 'success' : 'error'" :text="systemStatus.kafka ? '已连接' : '未连接'" />
                        </div>
                      </div>
                    </a-col>
                  </a-row>
                </a-card>
              </div>
            </div>
          </a-card>
        </div>
      </a-layout-content>
      
      <a-layout-footer class="footer">
        <div class="footer-content">
          <span>Tauri 设备信号网关 ©2024 由 Rust + Vue 3 构建</span>
          <a-space>
            <a href="https://tauri.app" target="_blank">Tauri 官网</a>
            <a href="https://github.com" target="_blank">项目源码</a>
          </a-space>
        </div>
      </a-layout-footer>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  HomeOutlined,
  DashboardOutlined,
  SettingOutlined,
  ApiOutlined,
  RocketOutlined,
  DatabaseOutlined,
  LineChartOutlined,
  CloudOutlined
} from '@ant-design/icons-vue'
import { message } from 'ant-design-vue'

// 系统状态
const systemStatus = ref({
  mariadb: false,
  influxdb: false,
  kafka: false
})

// 检查系统状态
const checkSystemStatus = async () => {
  try {
    const results = await Promise.allSettled([
      invoke('test_connection', { service: 'mariadb' }),
      invoke('test_connection', { service: 'influxdb' }),
      invoke('test_connection', { service: 'kafka' })
    ])
    
    systemStatus.value.mariadb = results[0].status === 'fulfilled' ? results[0].value as boolean : false
    systemStatus.value.influxdb = results[1].status === 'fulfilled' ? results[1].value as boolean : false
    systemStatus.value.kafka = results[2].status === 'fulfilled' ? results[2].value as boolean : false
  } catch (error) {
    console.error('系统状态检查失败:', error)
  }
}

onMounted(() => {
  checkSystemStatus()
})
</script>

<style scoped>
.home-container {
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
}

.welcome-section {
  max-width: 1200px;
  margin: 0 auto;
}

.welcome-card {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 20px;
}

.intro-content {
  padding: 16px 0;
}

.intro-text {
  font-size: 16px;
  margin-bottom: 24px;
  text-align: center;
  color: #666;
}

.feature-grid {
  margin: 24px 0;
}

.feature-card {
  text-align: center;
  height: 120px;
}

.action-section {
  text-align: center;
  margin: 32px 0;
}

.greet-section {
  text-align: center;
  margin-top: 24px;
}

.greet-result {
  margin-top: 16px;
  max-width: 400px;
  margin-left: auto;
  margin-right: auto;
}

.system-status {
  margin-top: 24px;
}

.status-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.status-card :deep(.ant-card-head-title) {
  color: white;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
}

.status-info {
  flex: 1;
}

.status-title {
  font-weight: 500;
  margin-bottom: 4px;
  color: white;
}

.footer {
  text-align: center;
  background: #001529;
  color: white;
}

.footer-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.footer-content a {
  color: #1890ff;
}
</style>
