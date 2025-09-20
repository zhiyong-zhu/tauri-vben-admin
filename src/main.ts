import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'
import App from './App.vue'
import 'ant-design-vue/dist/reset.css'

// 创建路由
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: () => import('./views/Home.vue')
    },
    {
      path: '/dashboard',
      name: 'Dashboard', 
      component: () => import('./views/Dashboard.vue')
    }
  ]
})

// 创建Pinia状态管理
const pinia = createPinia()

// 创建Vue应用
const app = createApp(App)

app.use(router)
app.use(pinia)

app.mount('#app')
