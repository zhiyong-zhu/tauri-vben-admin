# 🪟 Windows 构建指南

## 问题总结
在 macOS 上为 Windows 交叉编译 Tauri 应用时遇到 `llvm-rc` 工具缺失的问题。

## 🎯 推荐解决方案

### 方案1：GitHub Actions 自动构建 (最推荐)

**优点：**
- ✅ 无需本地配置复杂环境
- ✅ 自动为所有平台构建
- ✅ 免费使用 GitHub 提供的 Windows 虚拟机
- ✅ 可以同时构建 Windows、macOS、Linux 版本

**步骤：**
1. 将代码推送到 GitHub
2. 创建 GitHub Actions 工作流
3. 自动构建并发布所有平台版本

### 方案2：Windows 虚拟机

**使用工具：**
- Parallels Desktop (付费，性能好)
- VMware Fusion (付费)
- VirtualBox (免费，性能一般)

**步骤：**
1. 安装 Windows 虚拟机
2. 在虚拟机中安装开发环境
3. 直接在 Windows 中构建

### 方案3：云服务器

**使用服务：**
- AWS EC2 Windows 实例
- Azure Windows 虚拟机
- 阿里云 Windows 服务器

### 方案4：修复交叉编译 (高级用户)

如果你想继续尝试交叉编译，需要：

```bash
# 1. 安装完整的 LLVM 工具链
brew install llvm

# 2. 设置环境变量
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export AR_x86_64_pc_windows_msvc="llvm-ar"
export CC_x86_64_pc_windows_msvc="clang"
export CXX_x86_64_pc_windows_msvc="clang++"

# 3. 可能还需要安装 Windows SDK (复杂)
```

## 🚀 GitHub Actions 配置示例

创建 `.github/workflows/build.yml`：

```yaml
name: 'Build Tauri App'
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build-tauri:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Rust setup
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install frontend dependencies
        run: pnpm install

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: app-v__VERSION__
          releaseName: 'Tauri Vben Admin v__VERSION__'
          releaseBody: 'Cross-platform desktop application built with Tauri + Vue 3 + Vben Admin'
          releaseDraft: true
          prerelease: false
```

## 📋 各方案对比

| 方案 | 难度 | 成本 | 推荐度 | 说明 |
|------|------|------|--------|------|
| GitHub Actions | ⭐ | 免费 | ⭐⭐⭐⭐⭐ | 最推荐，简单可靠 |
| 虚拟机 | ⭐⭐ | 中等 | ⭐⭐⭐ | 本地控制，需要 Windows 许可证 |
| 云服务器 | ⭐⭐ | 按使用付费 | ⭐⭐⭐ | 灵活，按需使用 |
| 交叉编译 | ⭐⭐⭐⭐ | 免费 | ⭐⭐ | 复杂，容易出问题 |

## 🎯 建议

对于你的情况，我强烈推荐使用 **GitHub Actions**：

1. **简单** - 只需要创建一个配置文件
2. **可靠** - GitHub 提供的环境很稳定
3. **免费** - 公开仓库免费使用
4. **全平台** - 一次配置，构建所有平台

你现在已经有了完整的 macOS 版本，使用 GitHub Actions 可以轻松获得 Windows 和 Linux 版本！
