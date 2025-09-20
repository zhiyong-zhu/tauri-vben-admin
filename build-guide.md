# 🚀 Tauri Vben Admin 跨平台构建指南

## 📱 关于 Tauri 应用特性

### ✅ Tauri 是什么
- **桌面应用框架** - 创建原生桌面应用程序
- **跨平台支持** - Windows、macOS、Linux
- **高性能** - 基于 Rust 后端 + Web 前端
- **小体积** - 比 Electron 应用更小

### ❌ Tauri 不是什么
- **不是 Web 服务器** - 不支持远程浏览器访问
- **不是 PWA** - 不能直接在浏览器中运行
- **不是移动应用** - 主要针对桌面平台

## 🛠️ 构建命令

### 当前平台构建
```bash
# 开发模式
pnpm tauri dev

# 生产构建
pnpm tauri build
```

### 跨平台构建

#### 🍎 macOS 构建
```bash
# 在 macOS 上构建
pnpm tauri build

# 生成文件:
# - Tauri Vben Admin.app (应用程序)
# - Tauri Vben Admin_1.0.0_aarch64.dmg (安装包)
```

#### 🪟 Windows 构建
```bash
# 在 Windows 上或使用交叉编译
pnpm tauri build --target x86_64-pc-windows-msvc

# 生成文件:
# - Tauri Vben Admin.exe
# - Tauri Vben Admin_1.0.0_x64_en-US.msi
```

#### 🐧 Linux 构建
```bash
# 在 Linux 上构建
pnpm tauri build

# 生成文件:
# - tauri-vben-admin (可执行文件)
# - tauri-vben-admin_1.0.0_amd64.deb (Debian 包)
# - tauri-vben-admin_1.0.0_amd64.AppImage (AppImage)
```

## 📦 已构建的文件

### macOS (当前构建)
✅ **应用程序**: `Tauri Vben Admin.app`
✅ **安装包**: `Tauri Vben Admin_1.0.0_aarch64.dmg`

位置: `/Users/zhuzhiyong/code/tauri-app/src-tauri/target/release/bundle/`

## 🔧 交叉编译设置

### 为其他平台构建 (高级)

#### Windows 目标 (在 macOS/Linux 上)
```bash
# 安装 Windows 目标
rustup target add x86_64-pc-windows-msvc

# 构建 Windows 版本
pnpm tauri build --target x86_64-pc-windows-msvc
```

#### Linux 目标 (在 macOS/Windows 上)
```bash
# 安装 Linux 目标
rustup target add x86_64-unknown-linux-gnu

# 构建 Linux 版本
pnpm tauri build --target x86_64-unknown-linux-gnu
```

## 📋 支持的平台和格式

| 平台 | 可执行文件 | 安装包格式 |
|------|------------|------------|
| **macOS** | `.app` | `.dmg` |
| **Windows** | `.exe` | `.msi`, `.nsis` |
| **Linux** | 二进制文件 | `.deb`, `.rpm`, `.AppImage` |

## 🎯 分发建议

### macOS
- 使用 `.dmg` 文件分发
- 可能需要代码签名 (Apple Developer Account)

### Windows  
- 使用 `.msi` 安装包
- 可能需要代码签名证书

### Linux
- `.AppImage` - 便携式，无需安装
- `.deb` - 适用于 Ubuntu/Debian
- `.rpm` - 适用于 Red Hat/Fedora

## 🔐 代码签名 (可选)

### macOS 代码签名
```bash
# 需要 Apple Developer Account
export APPLE_CERTIFICATE="Developer ID Application: Your Name"
export APPLE_CERTIFICATE_PASSWORD="your-password"
pnpm tauri build
```

### Windows 代码签名
```bash
# 需要代码签名证书
export WINDOWS_CERTIFICATE_THUMBPRINT="your-thumbprint"
pnpm tauri build
```

## 🚀 自动化构建 (GitHub Actions)

可以设置 GitHub Actions 来自动构建所有平台:

```yaml
# .github/workflows/build.yml
name: Build Tauri App
on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v2
      - name: Setup Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '18'
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install dependencies
        run: pnpm install
      - name: Build Tauri app
        run: pnpm tauri build
```

## 📝 注意事项

1. **本地应用** - Tauri 应用是桌面程序，不能在浏览器中访问
2. **平台特定** - 每个平台需要在对应系统上构建 (除非使用交叉编译)
3. **依赖要求** - 用户机器需要满足系统要求
4. **更新机制** - 可以集成 Tauri 的自动更新功能

## 🎉 总结

你的 Tauri + Vben Admin 应用现在可以:
- ✅ 在 macOS 上作为原生应用运行
- ✅ 打包为 `.app` 和 `.dmg` 格式
- ✅ 使用相同方法为 Windows 和 Linux 构建
- ✅ 提供现代化的管理后台界面
- ✅ 享受 Rust 的高性能和安全性
