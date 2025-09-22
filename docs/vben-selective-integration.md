# Vben 选择性集成方案

## 🎯 目标功能
- ✅ 高级组件（表格、表单、图表等）
- ✅ 国际化系统
- ❌ 不要完整框架架构

## 方案 1：源码提取（推荐）

### 步骤 1：克隆 Vben 项目
```bash
# 临时克隆用于提取代码
git clone https://github.com/vbenjs/vue-vben-admin.git vben-temp
cd vben-temp
```

### 步骤 2：提取需要的组件
```bash
# 复制高级组件到我们的项目
cp -r packages/@core/ui-kit/shadcn-ui/src/components/* ../src/components/vben/
cp -r apps/web-antd/src/components/* ../src/components/business/

# 复制国际化相关
cp -r packages/locales/* ../src/locales/
cp -r packages/@core/preferences/src/config/* ../src/config/
```

### 步骤 3：复制样式和工具
```bash
# 样式文件
cp -r packages/@core/design/src/* ../src/styles/vben/

# 工具函数
cp -r packages/utils/src/* ../src/utils/vben/

# hooks
cp -r packages/hooks/src/* ../src/hooks/vben/
```

## 方案 2：使用第三方包

### 可用的相关包
```bash
# 国际化
npm install vben-locales

# hooks 和工具
npm install vben-hooks
npm install vben-aggregate-utils

# 样式
npm install vben-aggregate-styles
```

## 方案 3：手动实现类似功能

### 国际化方案
```bash
npm install vue-i18n@next
```

### 高级组件方案
```bash
# 使用其他优秀的组件库
npm install @arco-design/web-vue    # 字节跳动的组件库
npm install naive-ui                # 尤雨溪推荐的组件库
npm install tdesign-vue-next        # 腾讯的组件库
```
