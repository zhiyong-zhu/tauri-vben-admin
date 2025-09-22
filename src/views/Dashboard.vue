<template>
  <div class="dashboard-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">设备信号网关</span>
        </div>
        <a-menu
          theme="dark"
          mode="horizontal"
          :default-selected-keys="['2']"
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
        <div class="dashboard-content">
          <a-page-header
            class="page-header"
            title="仪表盘"
            sub-title="设备信号网关 - 实时监控与数据统计"
          />
          
          <!-- 统计卡片 -->
          <a-row :gutter="[16, 16]" class="stats-row">
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="连接设备数"
                  :value="deviceStats.totalDevices"
                  :value-style="{ color: '#3f8600' }"
                >
                  <template #prefix>
                    <MobileOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="今日信号数"
                  :value="deviceStats.todaySignals"
                  :value-style="{ color: '#1890ff' }"
                >
                  <template #prefix>
                    <SignalFilled />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="在线设备"
                  :value="deviceStats.onlineDevices"
                  :value-style="{ color: '#722ed1' }"
                >
                  <template #prefix>
                    <WifiOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="系统状态"
                  :value="systemStatus.healthy ? '正常' : '异常'"
                  :value-style="{ color: systemStatus.healthy ? '#52c41a' : '#ff4d4f' }"
                >
                  <template #prefix>
                    <CheckCircleOutlined v-if="systemStatus.healthy" />
                    <ExclamationCircleOutlined v-else />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
          </a-row>
          
          <!-- 图表和表格区域 -->
          <a-row :gutter="[16, 16]" class="charts-row">
            <a-col :span="12">
              <a-card title="信号趋势" class="chart-card">
                <div class="chart-placeholder">
                  <BarChartOutlined style="font-size: 48px; color: #d9d9d9;" />
                  <p style="margin-top: 16px; color: #999;">设备信号量趋势图 (可集成 ECharts)</p>
                </div>
              </a-card>
            </a-col>
            
            <a-col :span="12">
              <a-card title="设备类型分布" class="chart-card">
                <div class="chart-placeholder">
                  <PieChartOutlined style="font-size: 48px; color: #d9d9d9;" />
                  <p style="margin-top: 16px; color: #999;">设备类型分布图 (可集成 ECharts)</p>
                </div>
              </a-card>
            </a-col>
          </a-row>
          
          <!-- 数据表格 -->
          <a-row class="table-row">
            <a-col :span="24">
              <a-card title="最新设备信号" class="table-card">
                <template #extra>
                  <a-button @click="loadLatestSignals" :loading="loading">刷新</a-button>
                </template>
                <a-table
                  :columns="tableColumns"
                  :data-source="latestSignals"
                  :pagination="{ pageSize: 8 }"
                  size="middle"
                  :loading="loading"
                >
                  <template #bodyCell="{ column, record }">
                    <template v-if="column.key === 'signal_type'">
                      <a-tag color="blue">{{ record.signal_type }}</a-tag>
                    </template>
                    <template v-if="column.key === 'value'">
                      <span style="font-weight: 500;">{{ record.value }} {{ record.unit || '' }}</span>
                    </template>
                    <template v-if="column.key === 'timestamp'">
                      {{ formatTimestamp(record.timestamp) }}
                    </template>
                    <template v-if="column.key === 'action'">
                      <a-space>
                        <a-button type="link" size="small">查看</a-button>
                        <a-button type="link" size="small">详情</a-button>
                      </a-space>
                    </template>
                  </template>
                </a-table>
              </a-card>
            </a-col>
          </a-row>
        </div>
      </a-layout-content>
      
      <a-layout-footer class="footer">
        <div class="footer-content">
          <span>Tauri 设备信号网关 ©2024 由 Rust + Vue 3 构建</span>
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
  MobileOutlined,
  SignalFilled,
  WifiOutlined,
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  BarChartOutlined,
  PieChartOutlined
} from '@ant-design/icons-vue'
import { message } from 'ant-design-vue'

// 设备统计数据
const deviceStats = ref({
  totalDevices: 0,
  onlineDevices: 0,
  todaySignals: 0
})

// 系统状态
const systemStatus = ref({
  healthy: true,
  mariadb: false,
  influxdb: false,
  kafka: false
})

// 最新设备信号数据
const latestSignals = ref<any[]>([])

// 加载状态
const loading = ref(false)

// 表格列定义
const tableColumns = [
  {
    title: '设备ID',
    dataIndex: 'device_id',
    key: 'device_id',
  },
  {
    title: '信号类型',
    dataIndex: 'signal_type',
    key: 'signal_type',
  },
  {
    title: '数值',
    dataIndex: 'value',
    key: 'value',
  },
  {
    title: '时间',
    dataIndex: 'timestamp',
    key: 'timestamp',
  },
  {
    title: '操作',
    key: 'action',
  },
]

// 时间格式化
const formatTimestamp = (timestamp: string) => {
  return new Date(timestamp).toLocaleString('zh-CN')
}

// 加载最新信号数据
const loadLatestSignals = async () => {
  loading.value = true
  try {
    const signals = await invoke('get_latest_device_signals', { limit: 20 })
    latestSignals.value = signals as any[]
  } catch (error) {
    console.error('加载信号数据失败:', error)
    message.error('加载信号数据失败')
  } finally {
    loading.value = false
  }
}

// 加载统计数据
const loadStats = async () => {
  try {
    // 模拟统计数据，实际可通过API获取
    deviceStats.value = {
      totalDevices: Math.floor(Math.random() * 100) + 50,
      onlineDevices: Math.floor(Math.random() * 80) + 30,
      todaySignals: Math.floor(Math.random() * 5000) + 1000
    }
    
    // 检查系统状态
    const results = await Promise.allSettled([
      invoke('test_connection', { service: 'mariadb' }),
      invoke('test_connection', { service: 'influxdb' }),
      invoke('test_connection', { service: 'kafka' })
    ])
    
    systemStatus.value.mariadb = results[0].status === 'fulfilled' ? results[0].value as boolean : false
    systemStatus.value.influxdb = results[1].status === 'fulfilled' ? results[1].value as boolean : false
    systemStatus.value.kafka = results[2].status === 'fulfilled' ? results[2].value as boolean : false
    systemStatus.value.healthy = systemStatus.value.mariadb && systemStatus.value.influxdb && systemStatus.value.kafka
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

onMounted(() => {
  loadStats()
  loadLatestSignals()
})
</script>

<style scoped>
.dashboard-container {
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

.dashboard-content {
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  background: white;
  margin-bottom: 24px;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.stats-row {
  margin-bottom: 24px;
}

.stat-card {
  text-align: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.charts-row {
  margin-bottom: 24px;
}

.chart-card {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.chart-placeholder {
  height: 200px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: #fafafa;
  border-radius: 6px;
}

.table-row {
  margin-bottom: 24px;
}

.table-card {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.footer {
  text-align: center;
  background: #001529;
  color: white;
}

.footer-content {
  max-width: 1200px;
  margin: 0 auto;
}
</style>
