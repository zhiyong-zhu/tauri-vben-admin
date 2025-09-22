<template>
  <div class="api-test-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">设备信号网关 - API测试</span>
        </div>
        <a-menu
          theme="dark"
          mode="horizontal"
          :default-selected-keys="['4']"
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
        <div class="api-test-content">
          <a-page-header
            class="page-header"
            title="API测试"
            sub-title="测试设备信号网关的各种API接口"
          />

          <a-row :gutter="[24, 24]">
            <!-- 左侧：API列表 -->
            <a-col :span="8">
              <a-card title="API接口列表" class="api-list-card">
                <a-menu
                  v-model:selectedKeys="selectedApiKeys"
                  mode="vertical"
                  @click="handleApiSelect"
                >
                  <a-menu-item key="health">
                    <template #icon>
                      <HeartOutlined />
                    </template>
                    健康检查
                  </a-menu-item>
                  <a-menu-item key="status">
                    <template #icon>
                      <InfoCircleOutlined />
                    </template>
                    系统状态
                  </a-menu-item>
                  <a-menu-item key="signal">
                    <template #icon>
                      <SendOutlined />
                    </template>
                    发送信号
                  </a-menu-item>
                  <a-menu-item key="batch-signal">
                    <template #icon>
                      <BlockOutlined />
                    </template>
                    批量发送信号
                  </a-menu-item>
                  <a-menu-item key="get-latest">
                    <template #icon>
                      <ClockCircleOutlined />
                    </template>
                    获取最新信号
                  </a-menu-item>
                  <a-menu-item key="get-device">
                    <template #icon>
                      <MobileOutlined />
                    </template>
                    获取设备信号
                  </a-menu-item>
                  <a-menu-item key="test-kafka">
                    <template #icon>
                      <CloudOutlined />
                    </template>
                    测试Kafka
                  </a-menu-item>
                </a-menu>
              </a-card>
            </a-col>

            <!-- 右侧：API测试区域 -->
            <a-col :span="16">
              <a-card :title="currentApi.title" class="api-test-card">
                <!-- API信息 -->
                <div class="api-info">
                  <a-descriptions :column="1" size="small" bordered>
                    <a-descriptions-item label="请求方法">
                      <a-tag :color="getMethodColor(currentApi.method)">
                        {{ currentApi.method }}
                      </a-tag>
                    </a-descriptions-item>
                    <a-descriptions-item label="请求路径">
                      <a-typography-text code>{{ currentApi.path }}</a-typography-text>
                    </a-descriptions-item>
                    <a-descriptions-item label="说明">
                      {{ currentApi.description }}
                    </a-descriptions-item>
                  </a-descriptions>
                </div>

                <!-- 请求参数 -->
                <div class="request-section" v-if="currentApi.hasParams">
                  <h4>请求参数</h4>
                  <div v-if="currentApi.key === 'signal'">
                    <a-form layout="vertical">
                      <a-row :gutter="16">
                        <a-col :span="12">
                          <a-form-item label="设备ID">
                            <a-input v-model:value="signalForm.device_id" placeholder="sensor_001" />
                          </a-form-item>
                        </a-col>
                        <a-col :span="12">
                          <a-form-item label="信号类型">
                            <a-input v-model:value="signalForm.signal_type" placeholder="temperature" />
                          </a-form-item>
                        </a-col>
                      </a-row>
                      <a-row :gutter="16">
                        <a-col :span="12">
                          <a-form-item label="值">
                            <a-input-number v-model:value="signalForm.value" :precision="2" style="width: 100%" />
                          </a-form-item>
                        </a-col>
                        <a-col :span="12">
                          <a-form-item label="单位（可选）">
                            <a-input v-model:value="signalForm.unit" placeholder="°C" />
                          </a-form-item>
                        </a-col>
                      </a-row>
                      <a-form-item label="元数据（可选，JSON格式）">
                        <a-textarea v-model:value="signalForm.metadata" :rows="3" placeholder='{"location": "room_1"}' />
                      </a-form-item>
                    </a-form>
                  </div>
                  
                  <div v-else-if="currentApi.key === 'batch-signal'">
                    <a-form layout="vertical">
                      <a-form-item label="批量信号数据（JSON数组格式）">
                        <a-textarea v-model:value="batchSignalForm" :rows="8" :placeholder="batchSignalPlaceholder" />
                      </a-form-item>
                    </a-form>
                  </div>

                  <div v-else-if="currentApi.key === 'get-latest'">
                    <a-form layout="vertical">
                      <a-form-item label="限制数量">
                        <a-input-number v-model:value="getLatestForm.limit" :min="1" :max="1000" style="width: 100%" />
                      </a-form-item>
                    </a-form>
                  </div>

                  <div v-else-if="currentApi.key === 'get-device'">
                    <a-form layout="vertical">
                      <a-row :gutter="16">
                        <a-col :span="12">
                          <a-form-item label="设备ID">
                            <a-input v-model:value="getDeviceForm.device_id" placeholder="sensor_001" />
                          </a-form-item>
                        </a-col>
                        <a-col :span="12">
                          <a-form-item label="限制数量">
                            <a-input-number v-model:value="getDeviceForm.limit" :min="1" :max="1000" style="width: 100%" />
                          </a-form-item>
                        </a-col>
                      </a-row>
                    </a-form>
                  </div>

                  <div v-else-if="currentApi.key === 'test-kafka'">
                    <a-form layout="vertical">
                      <a-row :gutter="16">
                        <a-col :span="12">
                          <a-form-item label="消息键">
                            <a-input v-model:value="kafkaTestForm.key" placeholder="test_key" />
                          </a-form-item>
                        </a-col>
                        <a-col :span="12">
                          <a-form-item label="消息内容">
                            <a-input v-model:value="kafkaTestForm.message" placeholder="Hello Kafka!" />
                          </a-form-item>
                        </a-col>
                      </a-row>
                    </a-form>
                  </div>
                </div>

                <!-- 发送请求按钮 -->
                <div class="action-section">
                  <a-button type="primary" size="large" @click="sendRequest" :loading="loading">
                    <SendOutlined />
                    发送请求
                  </a-button>
                  <a-button @click="clearResponse" style="margin-left: 12px;">
                    <ClearOutlined />
                    清空响应
                  </a-button>
                </div>

                <!-- 响应结果 -->
                <div class="response-section" v-if="response">
                  <h4>响应结果</h4>
                  <a-alert
                    :type="response.success ? 'success' : 'error'"
                    :message="`HTTP ${response.status} - ${response.success ? '请求成功' : '请求失败'}`"
                    style="margin-bottom: 16px;"
                  />
                  <a-typography-paragraph>
                    <pre class="response-content">{{ JSON.stringify(response.data, null, 2) }}</pre>
                  </a-typography-paragraph>
                </div>
              </a-card>
            </a-col>
          </a-row>

          <!-- API文档卡片 -->
          <a-card title="API文档" style="margin-top: 24px;">
            <a-collapse>
              <a-collapse-panel key="endpoints" header="所有API端点">
                <div class="api-doc">
                  <h4>基础URL</h4>
                  <p><code>http://localhost:8080/api</code></p>
                  
                  <h4>API端点列表</h4>
                  <a-table :columns="docColumns" :data-source="docData" :pagination="false" size="small">
                    <template #bodyCell="{ column, record }">
                      <template v-if="column.key === 'method'">
                        <a-tag :color="getMethodColor(record.method)">{{ record.method }}</a-tag>
                      </template>
                      <template v-if="column.key === 'path'">
                        <code>{{ record.path }}</code>
                      </template>
                    </template>
                  </a-table>
                </div>
              </a-collapse-panel>
              
              <a-collapse-panel key="examples" header="请求示例">
                <div class="api-examples">
                  <h4>curl 示例</h4>
                  <a-typography-paragraph>
                    <pre class="code-example">
# 发送单个信号
curl -X POST http://localhost:8080/api/signals \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "sensor_001",
    "signal_type": "temperature",
    "value": 25.5,
    "unit": "°C"
  }'

# 健康检查
curl http://localhost:8080/api/health

# 获取最新信号
curl http://localhost:8080/api/signals/latest?limit=10</pre>
                  </a-typography-paragraph>
                </div>
              </a-collapse-panel>
            </a-collapse>
          </a-card>
        </div>
      </a-layout-content>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { invoke } from '@tauri-apps/api/core'
import {
  HomeOutlined,
  DashboardOutlined,
  SettingOutlined,
  ApiOutlined,
  HeartOutlined,
  InfoCircleOutlined,
  SendOutlined,
  BlockOutlined,
  ClockCircleOutlined,
  MobileOutlined,
  CloudOutlined,
  ClearOutlined
} from '@ant-design/icons-vue'

// API定义
const apiList = [
  {
    key: 'health',
    title: '健康检查',
    method: 'GET',
    path: '/api/health',
    description: '检查所有服务的连接状态',
    hasParams: false
  },
  {
    key: 'status',
    title: '系统状态',
    method: 'GET',
    path: '/api/status',
    description: '获取系统基本信息',
    hasParams: false
  },
  {
    key: 'signal',
    title: '发送设备信号',
    method: 'POST',
    path: '/api/signals',
    description: '发送单个设备信号数据',
    hasParams: true
  },
  {
    key: 'batch-signal',
    title: '批量发送信号',
    method: 'POST',
    path: '/api/signals/batch',
    description: '批量发送多个设备信号',
    hasParams: true
  },
  {
    key: 'get-latest',
    title: '获取最新信号',
    method: 'GET',
    path: '/api/signals/latest',
    description: '获取最新的设备信号数据',
    hasParams: true
  },
  {
    key: 'get-device',
    title: '获取设备信号',
    method: 'GET',
    path: '/api/signals/device/{device_id}',
    description: '获取指定设备的信号历史',
    hasParams: true
  },
  {
    key: 'test-kafka',
    title: '测试Kafka',
    method: 'POST',
    path: '/api/test/kafka',
    description: '发送测试消息到Kafka',
    hasParams: true
  }
]

// 状态管理
const selectedApiKeys = ref(['health'])
const loading = ref(false)
const response = ref<any>(null)

// 表单数据
const signalForm = ref({
  device_id: 'sensor_001',
  signal_type: 'temperature',
  value: 25.5,
  unit: '°C',
  metadata: '{"location": "room_1", "sensor_model": "DHT22"}'
})

const batchSignalForm = ref('')
const batchSignalPlaceholder = `[
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
]`

const getLatestForm = ref({
  limit: 10
})

const getDeviceForm = ref({
  device_id: 'sensor_001',
  limit: 10
})

const kafkaTestForm = ref({
  key: 'test_key',
  message: 'Hello from API Test!'
})

// 当前选中的API
const currentApi = computed(() => {
  return apiList.find(api => api.key === selectedApiKeys.value[0]) || apiList[0]
})

// 方法颜色映射
const getMethodColor = (method: string) => {
  const colors: Record<string, string> = {
    GET: 'blue',
    POST: 'green',
    PUT: 'orange',
    DELETE: 'red'
  }
  return colors[method] || 'default'
}

// API选择处理
const handleApiSelect = (info: any) => {
  selectedApiKeys.value = [info.key]
  clearResponse()
}

// 发送请求
const sendRequest = async () => {
  loading.value = true
  try {
    const api = currentApi.value
    
    // 对于可以直接通过Tauri处理的请求
    if (api.key === 'signal') {
      const signalData = { ...signalForm.value }
      if (signalData.metadata) {
        try {
          signalData.metadata = JSON.parse(signalData.metadata)
        } catch (e) {
          message.error('元数据JSON格式不正确')
          loading.value = false
          return
        }
      }
      
      try {
        const result = await invoke('send_test_signal', { signal: signalData })
        response.value = {
          status: 200,
          success: true,
          data: { message: result }
        }
        message.success('信号发送成功')
      } catch (error) {
        response.value = {
          status: 500,
          success: false,
          data: { error: error }
        }
        message.error('信号发送失败: ' + error)
      }
      loading.value = false
      return
    }

    // 其他API通过HTTP请求处理
    let url = `http://localhost:8080${api.path}`
    let options: RequestInit = {
      method: api.method,
      headers: {
        'Content-Type': 'application/json'
      }
    }

    // 构建请求参数 (其余代码保持不变)
    if (api.method === 'GET' && api.hasParams) {
      const params = new URLSearchParams()
      
      if (api.key === 'get-latest') {
        params.append('limit', getLatestForm.value.limit.toString())
      } else if (api.key === 'get-device') {
        url = url.replace('{device_id}', getDeviceForm.value.device_id)
        params.append('limit', getDeviceForm.value.limit.toString())
      }
      
      if (params.toString()) {
        url += '?' + params.toString()
      }
    } else if (api.method === 'POST' && api.hasParams) {
      let body: any = {}
      
      if (api.key === 'batch-signal') {
        try {
          body = JSON.parse(batchSignalForm.value || '[]')
        } catch (e) {
          message.error('批量数据JSON格式不正确')
          loading.value = false
          return
        }
      } else if (api.key === 'test-kafka') {
        body = { ...kafkaTestForm.value }
      }
      
      options.body = JSON.stringify(body)
    }

    // 发送请求
    const res = await fetch(url, options)
    const data = await res.json()

    response.value = {
      status: res.status,
      success: res.ok,
      data: data
    }

    if (res.ok) {
      message.success('请求成功')
    } else {
      message.error('请求失败')
    }
  } catch (error: any) {
    console.error('请求失败:', error)
    response.value = {
      status: 0,
      success: false,
      data: { error: error.message || '未知错误' }
    }
    message.error('请求失败: ' + (error.message || '未知错误'))
  } finally {
    loading.value = false
  }
}

// 清空响应
const clearResponse = () => {
  response.value = null
}

// 文档表格数据
const docColumns = [
  { title: '方法', dataIndex: 'method', key: 'method', width: 80 },
  { title: '路径', dataIndex: 'path', key: 'path' },
  { title: '说明', dataIndex: 'description', key: 'description' }
]

const docData = apiList.map((api, index) => ({
  key: index,
  method: api.method,
  path: api.path,
  description: api.description
}))

onMounted(() => {
  // 初始化批量信号表单
  batchSignalForm.value = batchSignalPlaceholder
})
</script>

<style scoped>
.api-test-container {
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

.api-test-content {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  background: white;
  margin-bottom: 24px;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.api-list-card,
.api-test-card {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  height: fit-content;
}

.api-info {
  margin-bottom: 24px;
}

.request-section,
.response-section {
  margin: 24px 0;
}

.request-section h4,
.response-section h4 {
  margin-bottom: 16px;
  color: #1890ff;
}

.action-section {
  text-align: center;
  margin: 24px 0;
  padding: 16px;
  background: #fafafa;
  border-radius: 6px;
}

.response-content {
  background: #f6f8fa;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  padding: 16px;
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.5;
  overflow-x: auto;
  max-height: 400px;
  overflow-y: auto;
}

.api-doc h4 {
  margin: 16px 0 8px 0;
  color: #1890ff;
}

.api-examples h4 {
  margin: 16px 0 8px 0;
  color: #1890ff;
}

.code-example {
  background: #f6f8fa;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  padding: 16px;
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.5;
  overflow-x: auto;
}

:deep(.ant-menu-vertical .ant-menu-item) {
  margin: 4px 0;
}

:deep(.ant-descriptions-item-label) {
  font-weight: 600;
}
</style>