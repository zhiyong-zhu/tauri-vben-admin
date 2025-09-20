<template>
  <div class="home-container">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">
          <img src="@/assets/tauri.svg" alt="Tauri" class="logo-img" />
          <span class="logo-text">Tauri + Vben Admin</span>
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
        </a-menu>
      </a-layout-header>
      
      <a-layout-content class="content">
        <div class="welcome-section">
          <a-card class="welcome-card">
            <template #title>
              <span class="card-title">
                <RocketOutlined />
                欢迎使用 Tauri + Vben Admin
              </span>
            </template>
            
            <div class="intro-content">
              <p class="intro-text">
                这是一个集成了 <strong>Tauri</strong> 和 <strong>Vben Admin</strong> 的现代化桌面应用程序。
              </p>
              
              <a-row :gutter="[16, 16]" class="feature-grid">
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <ThunderboltOutlined style="color: #1890ff;" />
                      高性能
                    </template>
                    <p>基于 Rust 的 Tauri 框架，提供原生级别的性能体验</p>
                  </a-card>
                </a-col>
                
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <SafetyOutlined style="color: #52c41a;" />
                      安全可靠
                    </template>
                    <p>内置安全机制，保护您的应用程序和用户数据</p>
                  </a-card>
                </a-col>
                
                <a-col :span="8">
                  <a-card size="small" class="feature-card">
                    <template #title>
                      <BugOutlined style="color: #722ed1;" />
                      现代化UI
                    </template>
                    <p>基于 Ant Design Vue 的精美界面设计</p>
                  </a-card>
                </a-col>
              </a-row>
              
              <div class="action-section">
                <a-space size="large">
                  <a-button type="primary" size="large" @click="handleGreet">
                    <UserOutlined />
                    测试 Tauri 调用
                  </a-button>
                  <a-button size="large" @click="$router.push('/dashboard')">
                    <DashboardOutlined />
                    前往仪表板
                  </a-button>
                </a-space>
              </div>
              
              <a-divider />
              
              <div class="greet-section">
                <a-input-group compact>
                  <a-input
                    v-model:value="greetName"
                    placeholder="输入您的名字..."
                    style="width: 200px"
                    @press-enter="handleGreet"
                  />
                  <a-button type="primary" @click="handleGreet">
                    问候
                  </a-button>
                </a-input-group>
                <div v-if="greetMessage" class="greet-result">
                  <a-alert :message="greetMessage" type="success" show-icon />
                </div>
              </div>
            </div>
          </a-card>
        </div>
      </a-layout-content>
      
      <a-layout-footer class="footer">
        <div class="footer-content">
          <span>Tauri + Vben Admin ©2024 Created with ❤️</span>
          <a-space>
            <a href="https://tauri.app" target="_blank">Tauri 官网</a>
            <a href="https://vben.vvbin.cn" target="_blank">Vben 文档</a>
          </a-space>
        </div>
      </a-layout-footer>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  HomeOutlined,
  DashboardOutlined,
  RocketOutlined,
  ThunderboltOutlined,
  SafetyOutlined,
  BugOutlined,
  UserOutlined
} from '@ant-design/icons-vue'
import { message } from 'ant-design-vue'

const greetName = ref('')
const greetMessage = ref('')

const handleGreet = async () => {
  if (!greetName.value.trim()) {
    message.warning('请输入您的名字')
    return
  }
  
  try {
    const result = await invoke('greet', { name: greetName.value })
    greetMessage.value = result as string
    message.success('调用成功!')
  } catch (error) {
    console.error('调用失败:', error)
    message.error('调用 Tauri 函数失败')
  }
}
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
