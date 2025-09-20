<template>
  <div class="dashboard-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">Tauri + Vben Admin</span>
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
        </a-menu>
      </a-layout-header>
      
      <a-layout-content class="content">
        <div class="dashboard-content">
          <a-page-header
            class="page-header"
            title="仪表板"
            sub-title="系统概览和数据统计"
          />
          
          <!-- 统计卡片 -->
          <a-row :gutter="[16, 16]" class="stats-row">
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="总用户数"
                  :value="1128"
                  :value-style="{ color: '#3f8600' }"
                >
                  <template #prefix>
                    <UserOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="活跃用户"
                  :value="893"
                  :value-style="{ color: '#1890ff' }"
                >
                  <template #prefix>
                    <TeamOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="今日访问"
                  :value="2456"
                  :value-style="{ color: '#722ed1' }"
                >
                  <template #prefix>
                    <EyeOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
            
            <a-col :span="6">
              <a-card class="stat-card">
                <a-statistic
                  title="系统状态"
                  value="正常"
                  :value-style="{ color: '#52c41a' }"
                >
                  <template #prefix>
                    <CheckCircleOutlined />
                  </template>
                </a-statistic>
              </a-card>
            </a-col>
          </a-row>
          
          <!-- 图表和表格区域 -->
          <a-row :gutter="[16, 16]" class="charts-row">
            <a-col :span="12">
              <a-card title="访问趋势" class="chart-card">
                <div class="chart-placeholder">
                  <BarChartOutlined style="font-size: 48px; color: #d9d9d9;" />
                  <p style="margin-top: 16px; color: #999;">图表区域 (可集成 ECharts)</p>
                </div>
              </a-card>
            </a-col>
            
            <a-col :span="12">
              <a-card title="用户分布" class="chart-card">
                <div class="chart-placeholder">
                  <PieChartOutlined style="font-size: 48px; color: #d9d9d9;" />
                  <p style="margin-top: 16px; color: #999;">饼图区域 (可集成 ECharts)</p>
                </div>
              </a-card>
            </a-col>
          </a-row>
          
          <!-- 数据表格 -->
          <a-row class="table-row">
            <a-col :span="24">
              <a-card title="最近活动" class="table-card">
                <a-table
                  :columns="tableColumns"
                  :data-source="tableData"
                  :pagination="{ pageSize: 5 }"
                  size="middle"
                >
                  <template #bodyCell="{ column, record }">
                    <template v-if="column.key === 'status'">
                      <a-tag :color="record.status === '成功' ? 'green' : 'orange'">
                        {{ record.status }}
                      </a-tag>
                    </template>
                    <template v-if="column.key === 'action'">
                      <a-space>
                        <a-button type="link" size="small">查看</a-button>
                        <a-button type="link" size="small">编辑</a-button>
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
          <span>Tauri + Vben Admin ©2024 Created with ❤️</span>
        </div>
      </a-layout-footer>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  HomeOutlined,
  DashboardOutlined,
  UserOutlined,
  TeamOutlined,
  EyeOutlined,
  CheckCircleOutlined,
  BarChartOutlined,
  PieChartOutlined
} from '@ant-design/icons-vue'

// 表格列定义
const tableColumns = [
  {
    title: '用户名',
    dataIndex: 'username',
    key: 'username',
  },
  {
    title: '操作',
    dataIndex: 'action',
    key: 'action',
  },
  {
    title: '时间',
    dataIndex: 'time',
    key: 'time',
  },
  {
    title: '状态',
    dataIndex: 'status',
    key: 'status',
  },
  {
    title: '操作',
    key: 'action',
  },
]

// 表格数据
const tableData = ref([
  {
    key: '1',
    username: '张三',
    action: '登录系统',
    time: '2024-01-20 14:30:00',
    status: '成功',
  },
  {
    key: '2',
    username: '李四',
    action: '修改密码',
    time: '2024-01-20 14:25:00',
    status: '成功',
  },
  {
    key: '3',
    username: '王五',
    action: '上传文件',
    time: '2024-01-20 14:20:00',
    status: '处理中',
  },
  {
    key: '4',
    username: '赵六',
    action: '删除数据',
    time: '2024-01-20 14:15:00',
    status: '成功',
  },
  {
    key: '5',
    username: '钱七',
    action: '导出报表',
    time: '2024-01-20 14:10:00',
    status: '成功',
  },
])
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
