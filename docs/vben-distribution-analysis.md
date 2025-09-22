# Vben 分发策略分析

## 🎯 Vben 不发布 NPM 包的原因

### 1. 📋 **模板项目定位**
- Vben 定位是"管理后台模板"，不是"组件库"
- 类似于 Vue CLI 创建的项目模板
- 用户通过克隆/下载源码开始开发

### 2. 🏗️ **Monorepo 架构**
- Vben 使用 pnpm workspace 管理多包
- 包之间高度耦合，不适合独立发布
- 内部包主要用于项目内部模块化

### 3. 🎨 **高度定制化**
- 每个项目都需要大量定制
- 发布通用包意义不大
- 更适合作为起始模板

### 4. 🔄 **快速迭代**
- 频繁更新，版本管理复杂
- 作为模板项目更灵活

## 📊 对比其他项目

### ✅ 发布 NPM 包的项目
- **Ant Design Vue** - 纯组件库
- **Element Plus** - 纯组件库
- **Naive UI** - 纯组件库

### ❌ 不发布 NPM 包的项目  
- **Vue Vben Admin** - 管理后台模板
- **Ant Design Pro** - 管理后台模板
- **Vue Element Admin** - 管理后台模板

## 🎯 实际使用方式

### Vben 官方推荐
```bash
# 方式 1：克隆源码
git clone https://github.com/vbenjs/vue-vben-admin.git

# 方式 2：使用脚手架
npm create vben@latest my-project

# 方式 3：下载 ZIP
# 从 GitHub 下载源码包
```

### 第三方包
```bash
# 一些开发者发布的 Vben 相关包
npm install vben-hooks           # 第三方提取的 hooks
npm install vben-aggregate-*     # 第三方聚合包
```
