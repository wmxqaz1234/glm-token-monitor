# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

PlanGuard 是一个跨平台桌面小工具，以像素风电子宠物形态展示 codingplan 的使用量。

- **Windows**: 96x96 透明悬浮窗，支持手动拖动
- **macOS**: 系统托盘 + 气泡面板（待实现）

## 常用命令

```bash
# 开发模式（热重载）
npm run tauri:dev

# 构建
npm run tauri:build

# 仅构建 Rust 部分
cd src-tauri && cargo build

# 仅运行已编译的二进制文件
./src-tauri/target/debug/plan-guard.exe  # Windows
./src-tauri/target/debug/plan-guard      # macOS/Linux

# 运行测试
cd src-tauri && cargo test
```

## 核心架构

### 数据流
```
Rust 端轮询 (5分钟) → fetch_usage() → Tauri Event ("usage-update") → Vue 监听 → UI 更新
```

### 状态机
基于 `usage_percent = used / total * 100`：

| 状态 | 百分比 | 颜色 |
|------|--------|------|
| Fresh | 0-30% | #10B981 (绿) |
| Flow | 31-60% | #3B82F6 (蓝) |
| Warning | 61-80% | #F59E0B (黄) |
| Panic | 81-95% | #F97316 (橙) |
| Dead | 96-100% | #6B7280 (灰) |

### 模块职责

**Rust 端 (`src-tauri/src/`)**
- `main.rs`: 入口，窗口初始化，平台特定配置
- `polling.rs`: 后台轮询服务，每 5 分钟获取使用量
- `events.rs`: 事件定义和数据结构 (`UsageData`)
- `commands.rs`: Tauri Commands (`get_current_usage`)
- `tray.rs`: macOS 系统托盘（待实现）

**Vue 端 (`src/`)**
- `composables/useUsageState.ts`: 状态计算逻辑（百分比、状态映射、颜色）
- `composables/useTauriEvents.ts`: 监听 Rust 端事件 (`usage-update`, `usage-error`)
- `composables/usePlatform.ts`: 平台检测（Windows/macOS）
- `components/PetWidget.vue`: Windows 悬浮窗组件（含手动拖动）
- `components/PopoverPanel.vue`: macOS 气泡面板（待实现）
- `App.vue`: 根组件，根据平台选择渲染组件

### 跨端通信

**Rust → Vue (Event)**
```rust
app.emit("usage-update", UsageData { used: 65, total: 100 })
```

```typescript
listen('usage-update', (event) => {
  usageData.value = event.payload
})
```

**Vue → Rust (Command)**
```typescript
invoke('get_current_usage')
```

## 平台特定注意事项

### Windows
- 窗口配置：`decorations: false`, `transparent: true`, `alwaysOnTop: true`
- 拖动实现：手动监听 `mousedown/mousemove/mouseup`，使用 `appWindow.setPosition()`
- 窗口大小：96x96 像素

### macOS
- 系统托盘图标需根据状态动态切换
- 不在 dock 栏显示
- 点击托盘显示气泡面板

## 开发约定

- API 数据格式：`{used: number, total: number}`
- 轮询间隔：5 分钟（硬编码在 `polling.rs`）
- 当前使用 Mock 数据，真实 API 集成待实现
- 所有状态相关逻辑集中在 `useUsageState.ts`
- 所有跨平台检测使用 `usePlatform.ts`

## 已知问题

- vue-tsc 在当前环境有兼容性问题，构建时可绕过
- macOS 系统托盘功能尚未实现
- 真实 API 集成待实现
