# PlanGuard 设计文档

**项目名称**：PlanGuard
**创建日期**：2026-04-04
**状态**：设计已批准

---

## 1. 项目概述

PlanGuard 是一个跨平台桌面小工具，以"像素风电子宠物"形态展示 codingplan 的使用量。

### 核心功能
- 定时请求远程 API 获取使用量（已用量/总用量）
- 根据用量百分比展示不同状态的宠物
- Windows：悬浮窗 + 拖拽支持
- macOS：系统托盘 + 气泡面板

### 技术栈
- **框架**：Tauri v2 (Rust)
- **前端**：Vue 3 + TypeScript + Vite
- **样式**：Tailwind CSS + 纯 CSS 动画
- **通信**：Tauri IPC (事件驱动)

---

## 2. 状态机设计

| 状态 | 百分比范围 | 颜色 | 动画 |
|------|-----------|------|------|
| Fresh | 0% - 30% | `#10B981` (绿) | 呼吸/吃东西 |
| Flow | 31% - 60% | `#3B82F6` (蓝) | 平稳敲打 |
| Warning | 61% - 80% | `#F59E0B` (黄) | 焦躁抖动 |
| Panic | 81% - 95% | `#F97316` (橙) | 冒汗抖动 |
| Dead | 96% - 100% | `#6B7280` (灰) | 宕机/灵魂出窍 |

---

## 3. 架构设计

### 3.1 项目结构

```
plan-guard/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # 入口，窗口/托盘初始化
│   │   ├── polling.rs           # 轮询服务模块
│   │   ├── commands.rs          # Tauri Commands
│   │   └── events.rs            # 事件定义
│   ├── icons/                   # 托盘图标（5 种状态）
│   └── tauri.conf.json          # Tauri 配置
├── src/
│   ├── main.ts                  # Vue 入口
│   ├── App.vue                  # 根组件
│   ├── components/
│   │   ├── PetWidget.vue        # 宠物组件（Windows）
│   │   ├── PopoverPanel.vue     # 气泡面板
│   │   └── StatusBar.vue        # 状态进度条
│   ├── composables/
│   │   ├── useUsageState.ts     # 状态计算逻辑
│   │   └── useTauriEvents.ts    # 事件监听
│   └── assets/
│       └── sprites/             # 雪碧图占位符
├── package.json
└── vite.config.ts
```

### 3.2 数据流

```
┌─────────────────────────────────────────────────────────────┐
│                         Rust 端                             │
├─────────────────────────────────────────────────────────────┤
│  tokio::time::interval (5分钟)                               │
│           │                                                 │
│           ▼                                                 │
│  fetch_usage() → Mock 返回 {used: 65, total: 100}           │
│           │                                                 │
│           ▼                                                 │
│  app.emit("usage-update", UsageData) ──────────────────┐    │
└─────────────────────────────────────────────────────────│────┘
                                                           │ IPC
┌──────────────────────────────────────────────────────────▼────┐
│                        Vue 端                                 │
├──────────────────────────────────────────────────────────────┤
│  listen("usage-update", callback)                             │
│           │                                                   │
│           ▼                                                   │
│  更新响应式数据 usageData                                     │
│           │                                                   │
│           ▼                                                   │
│  computed 计算 petState + stateColor                          │
│           │                                                   │
│           ▼                                                   │
│  UI 更新：宠物颜色、动画、进度条                               │
└──────────────────────────────────────────────────────────────┘
```

---

## 4. Rust 端设计

### 4.1 main.rs - 窗口/托盘初始化

```rust
// Windows: 创建 150x150 透明悬浮窗
#[cfg(target_os = "windows")]
fn create_floating_window() -> Window {
    // decorations: false
    // transparent: true
    // always_on_top: true
    // width: 150, height: 150
}

// macOS: 初始化系统托盘，隐藏 dock 图标
#[cfg(target_os = "macos")]
fn create_system_tray() -> TrayIcon {
    // 根据状态动态切换托盘图标
}
```

### 4.2 polling.rs - 轮询服务

```rust
use tokio::time::interval;

async fn fetch_usage() -> Result<UsageData> {
    // Mock 数据
    Ok(UsageData { used: 65, total: 100 })
}

async fn start_polling_loop(app: AppHandle) {
    let mut timer = interval(Duration::from_secs(300));
    loop {
        timer.tick().await;
        if let Ok(data) = fetch_usage().await {
            app.emit("usage-update", data)?;
        }
    }
}
```

### 4.3 events.rs - 事件定义

```rust
const EVENT_USAGE_UPDATE: &str = "usage-update";

#[derive(Clone, Serialize)]
struct UsageData {
    used: u32,
    total: u32,
}
```

### 4.4 commands.rs - Tauri Commands

```rust
#[tauri::command]
async fn get_current_usage() -> Result<UsageData, String> {
    fetch_usage().await.map_err(|e| e.to_string())
}
```

---

## 5. Vue 端设计

### 5.1 composables/useUsageState.ts

```typescript
import { computed } from 'vue'

const COLORS = {
  Fresh: '#10B981',
  Flow: '#3B82F6',
  Warning: '#F59E0B',
  Panic: '#F97316',
  Dead: '#6B7280'
}

export function useUsageState(used: Ref<number>, total: Ref<number>) {
  const usagePercent = computed(() => (used.value / total.value) * 100)

  const petState = computed<'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'>(() => {
    const p = usagePercent.value
    if (p <= 30) return 'Fresh'
    if (p <= 60) return 'Flow'
    if (p <= 80) return 'Warning'
    if (p <= 95) return 'Panic'
    return 'Dead'
  })

  const stateColor = computed(() => COLORS[petState.value])

  return { usagePercent, petState, stateColor }
}
```

### 5.2 composables/useTauriEvents.ts

```typescript
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'

export function useTauriEvents() {
  const usageData = ref({ used: 0, total: 100 })

  listen('usage-update', (event) => {
    usageData.value = event.payload as UsageData
  })

  return { usageData }
}
```

### 5.3 组件结构

**PetWidget.vue（Windows 悬浮窗）**
- 容器：`data-tauri-drag-region` 支持拖拽
- 宠物：80x80 色块 + 状态动画
- 悬浮交互：`@mouseenter/@mouseleave` 控制进度条

**StatusBar.vue（状态进度条）**
- 宽度动画过渡
- 百分比文字
- 颜色随状态变化

**PopoverPanel.vue（macOS 气泡面板）**
- 详细信息展示
- 托盘点击触发

---

## 6. 动画设计

### 6.1 状态动画

```css
@keyframes breathe {      /* Fresh */
  0%, 100% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}

@keyframes type {         /* Flow */
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

@keyframes shake {        /* Warning */
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

@keyframes sweat {        /* Panic */
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.02) rotate(-1deg); }
  75% { transform: scale(0.98) rotate(1deg); }
}

@keyframes ghost {        /* Dead */
  0% { transform: translateY(0); opacity: 1; }
  100% { transform: translateY(-20px); opacity: 0.3; }
}
```

### 6.2 悬浮交互

```css
.status-bar {
  width: 0;
  opacity: 0;
  transition: all 0.3s ease;
}

.pet-widget:hover .status-bar {
  width: 120px;
  opacity: 1;
}
```

---

## 7. 跨平台适配

### Windows
- 透明悬浮窗（150x150）
- 拖拽支持（`data-tauri-drag-region`）
- 悬浮展开进度条

### macOS
- 系统托盘图标（5 种状态）
- 气泡面板（点击托盘显示）
- 不在 dock 栏显示

---

## 8. 开发约定

- API 数据格式：`{used: number, total: number}`
- 轮询间隔：5 分钟（硬编码）
- 数据源：Mock 数据（暂不调用真实 API）
- 动画资源：色块占位符（后续替换为雪碧图）
- 开发环境：Windows 优先，代码需兼容 macOS

---

## 9. 后续扩展

- [ ] 添加配置文件支持（轮询间隔、API 端点）
- [ ] 替换占位符为真实雪碧图
- [ ] 集成真实 API
- [ ] 添加 API 认证支持
