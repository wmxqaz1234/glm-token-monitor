# PlanGuard 实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标：** 构建 PlanGuard 跨平台桌面小工具，以像素风电子宠物形态展示 codingplan 使用量

**架构：** 事件驱动架构 - Rust 端定时轮询并通过 Tauri Event 推送数据给 Vue 端，Vue 端监听事件并更新 UI 状态

**技术栈：** Tauri v2 (Rust), Vue 3 + TypeScript + Vite, Tailwind CSS

---

## 前置准备

### 环境要求
- Node.js 18+
- Rust 1.70+
- Tauri CLI: `cargo install tauri-cli`
- Windows 11 开发环境（代码需兼容 macOS）

---

## 任务 1：项目初始化

**文件：**
- 创建: `package.json`
- 创建: `tsconfig.json`
- 创建: `vite.config.ts`
- 创建: `src-tauri/Cargo.toml`
- 创建: `src-tauri/tauri.conf.json`
- 创建: `src-tauri/src/main.rs`
- 创建: `src/main.ts`
- 创建: `src/App.vue`
- 创建: `index.html`
- 创建: `.gitignore`

**Step 1: 初始化项目并创建 package.json**

创建 `package.json`:

```json
{
  "name": "plan-guard",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc && vite build",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build"
  },
  "dependencies": {
    "vue": "^3.4.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@vitejs/plugin-vue": "^5.0.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vue-tsc": "^1.8.0"
  }
}
```

**Step 2: 创建 TypeScript 配置**

创建 `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

创建 `tsconfig.node.json`:

```json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

**Step 3: 创建 Vite 配置**

创建 `vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
```

**Step 4: 创建 Tailwind CSS 配置**

创建 `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

创建 `postcss.config.js`:

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

**Step 5: 创建 Rust Cargo.toml**

创建 `src-tauri/Cargo.toml`:

```toml
[package]
name = "plan-guard"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2.0", features = ["tray-icon", "image-ico", "image-png"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

**Step 6: 创建 Tauri 配置**

创建 `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "PlanGuard",
  "version": "0.1.0",
  "identifier": "com.planguard.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "PlanGuard",
        "width": 150,
        "height": 150,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "resizable": false,
        "skipTaskbar": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": []
  }
}
```

**Step 7: 创建 build.rs**

创建 `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build()
}
```

**Step 8: 创建 Rust 入口文件**

创建 `src-tauri/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 9: 创建 HTML 入口**

创建 `index.html`:

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>PlanGuard</title>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

**Step 10: 创建 Vue 入口文件**

创建 `src/main.ts`:

```typescript
import { createApp } from 'vue'
import App from './App.vue'
import './style.css'

createApp(App).mount('#app')
```

创建 `src/style.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

#app {
  width: 100vw;
  height: 100vh;
}
```

**Step 11: 创建根组件**

创建 `src/App.vue`:

```vue
<script setup lang="ts">
// 根组件 - 后续将添加平台检测和条件渲染
</script>

<template>
  <div class="app-container">
    <h1 class="text-2xl font-bold">PlanGuard</h1>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}
</style>
```

**Step 12: 创建 .gitignore**

创建 `.gitignore`:

```
node_modules
dist
dist-src
target
.DS_Store
*.log
.vscode
.idea
```

**Step 13: 安装依赖并验证项目结构**

运行: `npm install`

预期: 成功安装所有依赖，无错误

**Step 14: 验证项目可以启动**

运行: `npm run tauri:dev`

预期: Tauri 开发服务器启动，显示 150x150 的窗口

**Step 15: 提交初始项目结构**

运行:
```bash
git add .
git commit -m "feat: initialize Tauri + Vue 3 project structure"
```

---

## 任务 2：Rust 端 - 事件定义模块

**文件：**
- 创建: `src-tauri/src/events.rs`

**Step 1: 创建事件模块文件**

创建 `src-tauri/src/events.rs`:

```rust
use serde::{Deserialize, Serialize};

/// 事件名称常量
pub const EVENT_USAGE_UPDATE: &str = "usage-update";

/// 使用量数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub used: u32,
    pub total: u32,
}

impl UsageData {
    /// 计算使用百分比
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_calculation() {
        let data = UsageData { used: 50, total: 100 };
        assert_eq!(data.percent(), 50.0);
    }

    #[test]
    fn test_percent_zero_total() {
        let data = UsageData { used: 10, total: 0 };
        assert_eq!(data.percent(), 0.0);
    }
}
```

**Step 2: 在 main.rs 中声明事件模块**

修改 `src-tauri/src/main.rs`，在文件顶部添加:

```rust
mod events;

use events::UsageData;
```

**Step 3: 运行 Rust 测试验证**

运行: `cd src-tauri && cargo test`

预期: 测试通过，验证百分比计算逻辑

**Step 4: 提交事件模块**

运行:
```bash
git add src-tauri/src/events.rs src-tauri/src/main.rs
git commit -m "feat: add event definitions and UsageData struct"
```

---

## 任务 3：Rust 端 - 轮询服务模块

**文件：**
- 创建: `src-tauri/src/polling.rs`

**Step 1: 创建轮询服务模块**

创建 `src-tauri/src/polling.rs`:

```rust
use crate::events::{UsageData, EVENT_USAGE_UPDATE};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

/// Mock 数据 - 返回固定的使用量数据
async fn fetch_usage() -> Result<UsageData, String> {
    // 模拟 API 请求延迟
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Mock 数据：65% 使用量
    Ok(UsageData {
        used: 65,
        total: 100,
    })
}

/// 启动轮询循环
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut timer = interval(Duration::from_secs(300)); // 5 分钟

    loop {
        timer.tick().await;

        match fetch_usage().await {
            Ok(data) => {
                if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                    eprintln!("Failed to emit usage update: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch usage: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_usage() {
        let result = fetch_usage().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.used, 65);
        assert_eq!(data.total, 100);
    }
}
```

**Step 2: 在 main.rs 中声明轮询模块**

修改 `src-tauri/src/main.rs`:

```rust
mod events;
mod polling;

use events::UsageData;
```

**Step 3: 运行 Rust 测试验证**

运行: `cd src-tauri && cargo test polling`

预期: 轮询模块测试通过

**Step 4: 提交轮询模块**

运行:
```bash
git add src-tauri/src/polling.rs src-tauri/src/main.rs
git commit -m "feat: add polling service with mock data"
```

---

## 任务 4：Rust 端 - Commands 模块

**文件：**
- 创建: `src-tauri/src/commands.rs`

**Step 1: 创建 Commands 模块**

创建 `src-tauri/src/commands.rs`:

```rust
use crate::events::UsageData;
use crate::polling::fetch_usage;

/// 手动获取当前使用量
#[tauri::command]
pub async fn get_current_usage() -> Result<UsageData, String> {
    fetch_usage().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current_usage() {
        let result = get_current_usage().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.used, 65);
    }
}
```

**Step 2: 在 main.rs 中注册 Commands**

修改 `src-tauri/src/main.rs`:

```rust
mod events;
mod polling;
mod commands;

use events::UsageData;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: 运行 Rust 测试验证**

运行: `cd src-tauri && cargo test commands`

预期: Commands 测试通过

**Step 4: 提交 Commands 模块**

运行:
```bash
git add src-tauri/src/commands.rs src-tauri/src/main.rs
git commit -m "feat: add Tauri commands for manual usage fetch"
```

---

## 任务 5：Rust 端 - 启动轮询服务

**文件：**
- 修改: `src-tauri/src/main.rs`

**Step 1: 在 main.rs 中启动轮询服务**

修改 `src-tauri/src/main.rs`:

```rust
mod events;
mod polling;
mod commands;

use events::UsageData;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage
        ])
        .setup(|app| {
            // 启动轮询服务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = polling::start_polling_loop(app_handle).await {
                    eprintln!("Polling loop error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
```

**Step 2: 验证编译**

运行: `cd src-tauri && cargo check`

预期: 编译成功，无错误

**Step 3: 启动应用验证轮询**

运行: `npm run tauri:dev`

预期: 应用启动，后台轮询服务运行（每 5 分钟）

**Step 4: 提交轮询服务启动**

运行:
```bash
git add src-tauri/src/main.rs
git commit -m "feat: start polling service on app startup"
```

---

## 任务 6：Vue 端 - 状态管理 Composable

**文件：**
- 创建: `src/composables/useUsageState.ts`

**Step 1: 创建状态计算逻辑**

创建 `src/composables/useUsageState.ts`:

```typescript
import { computed, type Ref } from 'vue'

export type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

export interface UsageData {
  used: number
  total: number
}

export const COLORS: Record<PetState, string> = {
  Fresh: '#10B981',
  Flow: '#3B82F6',
  Warning: '#F59E0B',
  Panic: '#F97316',
  Dead: '#6B7280'
}

export function useUsageState(used: Ref<number>, total: Ref<number>) {
  const usagePercent = computed(() => {
    if (total.value === 0) return 0
    return (used.value / total.value) * 100
  })

  const petState = computed<PetState>(() => {
    const p = usagePercent.value
    if (p <= 30) return 'Fresh'
    if (p <= 60) return 'Flow'
    if (p <= 80) return 'Warning'
    if (p <= 95) return 'Panic'
    return 'Dead'
  })

  const stateColor = computed(() => COLORS[petState.value])

  return {
    usagePercent,
    petState,
    stateColor
  }
}
```

**Step 2: 提交状态管理**

运行:
```bash
git add src/composables/useUsageState.ts
git commit -m "feat: add usage state calculation composable"
```

---

## 任务 7：Vue 端 - Tauri 事件监听 Composable

**文件：**
- 创建: `src/composables/useTauriEvents.ts`

**Step 1: 创建事件监听逻辑**

创建 `src/composables/useTauriEvents.ts`:

```typescript
import { ref, type Ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { UsageData } from './useUsageState'

export function useTauriEvents() {
  const usageData: Ref<UsageData> = ref({ used: 0, total: 100 })

  // 监听 Rust 端推送的 usage-update 事件
  const setupEventListener = async () => {
    const unlisten = await listen<UsageData>('usage-update', (event) => {
      usageData.value = event.payload
    })
    return unlisten
  }

  return {
    usageData,
    setupEventListener
  }
}
```

**Step 2: 提交事件监听**

运行:
```bash
git add src/composables/useTauriEvents.ts
git commit -m "feat: add Tauri event listener composable"
```

---

## 任务 8：Vue 端 - 宠物组件（基础结构）

**文件：**
- 创建: `src/components/PetWidget.vue`

**Step 1: 创建宠物组件基础结构**

创建 `src/components/PetWidget.vue`:

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { usagePercent, petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const isHovered = ref(false)

onMounted(async () => {
  await setupEventListener()
})
</script>

<template>
  <div
    class="pet-widget"
    data-tauri-drag-region
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="pet-container" :style="{ backgroundColor: stateColor }">
      <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
        <div class="eye left"></div>
        <div class="eye right"></div>
        <div class="mouth"></div>
      </div>
    </div>

    <div class="status-bar" :class="{ expanded: isHovered }">
      <div class="progress-track">
        <div class="progress-fill" :style="{ width: `${usagePercent}%`, backgroundColor: stateColor }"></div>
      </div>
      <div class="percent-text">{{ Math.round(usagePercent) }}%</div>
    </div>
  </div>
</template>

<style scoped>
.pet-widget {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
}

.pet-container {
  width: 80px;
  height: 80px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.5s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.pet-face {
  position: relative;
  width: 50px;
  height: 40px;
}

.eye {
  position: absolute;
  width: 8px;
  height: 8px;
  background: white;
  border-radius: 50%;
  top: 10px;
}

.eye.left { left: 8px; }
.eye.right { right: 8px; }

.mouth {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  width: 16px;
  height: 8px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 16px 16px;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 0;
  opacity: 0;
  overflow: hidden;
  transition: all 0.3s ease;
}

.status-bar.expanded {
  width: 120px;
  opacity: 1;
}

.progress-track {
  flex: 1;
  height: 6px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  transition: width 0.5s ease, background-color 0.5s ease;
}

.percent-text {
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}
</style>
```

**Step 2: 添加缺失的 computed 导入**

修改 `src/components/PetWidget.vue`，在 script 中添加:

```typescript
import { ref, computed, onMounted } from 'vue'
```

**Step 3: 在 App.vue 中使用 PetWidget**

修改 `src/App.vue`:

```vue
<script setup lang="ts">
import PetWidget from './components/PetWidget.vue'
</script>

<template>
  <div class="app-container">
    <PetWidget />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}
</style>
```

**Step 4: 提交宠物组件**

运行:
```bash
git add src/components/PetWidget.vue src/App.vue
git commit -m "feat: add PetWidget component with status bar"
```

---

## 任务 9：添加状态动画

**文件：**
- 修改: `src/components/PetWidget.vue`

**Step 1: 添加状态动画 CSS**

修改 `src/components/PetWidget.vue`，在 style 中添加:

```css
/* Fresh 呼吸动画 */
.state-fresh {
  animation: breathe 2s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}

/* Flow 敲打动画 */
.state-flow {
  animation: type 0.8s ease-in-out infinite;
}

@keyframes type {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* Warning 抖动动画 */
.state-warning {
  animation: shake 0.5s ease-in-out infinite;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

/* Panic 冒汗动画 */
.state-panic {
  animation: sweat 0.3s ease-in-out infinite;
}

@keyframes sweat {
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.02) rotate(-1deg); }
  75% { transform: scale(0.98) rotate(1deg); }
}

/* Dead 灵魂出窍动画 */
.state-dead {
  animation: ghost 2s ease-in-out infinite;
}

@keyframes ghost {
  0% { transform: translateY(0); opacity: 1; }
  50% { transform: translateY(-10px); opacity: 0.5; }
  100% { transform: translateY(0); opacity: 1; }
}
```

**Step 2: 提交状态动画**

运行:
```bash
git add src/components/PetWidget.vue
git commit -m "feat: add state animations for pet widget"
```

---

## 任务 10：添加平台检测

**文件：**
- 创建: `src/composables/usePlatform.ts`
- 修改: `src/App.vue`

**Step 1: 创建平台检测 Composable**

创建 `src/composables/usePlatform.ts`:

```typescript
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type Platform = 'windows' | 'macos' | 'linux' | 'unknown'

export function usePlatform() {
  const platform = ref<Platform>('unknown')

  onMounted(async () => {
    // 检测当前平台
    const arch = await invoke('__cmd__') as string
    if (navigator.userAgent.includes('Windows')) {
      platform.value = 'windows'
    } else if (navigator.userAgent.includes('Mac')) {
      platform.value = 'macos'
    } else if (navigator.userAgent.includes('Linux')) {
      platform.value = 'linux'
    }
  })

  const isWindows = computed(() => platform.value === 'windows')
  const isMacOS = computed(() => platform.value === 'macos')

  return {
    platform,
    isWindows,
    isMacOS
  }
}
```

**Step 2: 添加 computed 导入**

修改 `src/composables/usePlatform.ts`:

```typescript
import { ref, computed, onMounted } from 'vue'
```

**Step 3: 在 App.vue 中使用平台检测**

修改 `src/App.vue`:

```vue
<script setup lang="ts">
import PetWidget from './components/PetWidget.vue'
import { usePlatform } from './composables/usePlatform'

const { isWindows, isMacOS } = usePlatform()
</script>

<template>
  <div class="app-container">
    <PetWidget v-if="isWindows" />
    <div v-else-if="isMacOS" class="macos-placeholder">
      <p>macOS 托盘模式（待实现）</p>
    </div>
    <div v-else class="unknown-platform">
      <p>未知平台</p>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}

.macos-placeholder,
.unknown-platform {
  padding: 20px;
  text-align: center;
}
</style>
```

**Step 4: 提交平台检测**

运行:
```bash
git add src/composables/usePlatform.ts src/App.vue
git commit -m "feat: add platform detection for Windows/macOS"
```

---

## 任务 11：Windows 平台优化

**文件：**
- 修改: `src-tauri/tauri.conf.json`
- 修改: `src/style.css`

**Step 1: 优化 Windows 窗口配置**

修改 `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "PlanGuard",
  "version": "0.1.0",
  "identifier": "com.planguard.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "PlanGuard",
        "width": 150,
        "height": 150,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "resizable": false,
        "skipTaskbar": false,
        "center": true
      }
    ],
    "withGlobalTauri": true,
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": []
  }
}
```

**Step 2: 优化样式以支持透明背景**

修改 `src/style.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: transparent;
  user-select: none;
  -webkit-user-select: none;
}

#app {
  width: 100vw;
  height: 100vh;
  background: transparent;
}
```

**Step 3: 测试 Windows 模式**

运行: `npm run tauri:dev`

预期: Windows 下显示 150x150 透明悬浮窗，宠物可拖拽

**Step 4: 提交 Windows 优化**

运行:
```bash
git add src-tauri/tauri.conf.json src/style.css
git commit -m "feat: optimize Windows floating window settings"
```

---

## 任务 12：添加霓虹光晕效果

**文件：**
- 修改: `src/components/PetWidget.vue`

**Step 1: 添加光晕效果**

修改 `src/components/PetWidget.vue`，在 `.pet-container` 样式中添加:

```css
.pet-container {
  width: 80px;
  height: 80px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.5s ease, box-shadow 0.5s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1),
              0 0 20px var(--glow-color, rgba(16, 185, 129, 0.3));
}

/* 根据状态设置不同的光晕颜色 */
.state-fresh .pet-container {
  --glow-color: rgba(16, 185, 129, 0.4);
}

.state-flow .pet-container {
  --glow-color: rgba(59, 130, 246, 0.4);
}

.state-warning .pet-container {
  --glow-color: rgba(245, 158, 11, 0.4);
}

.state-panic .pet-container {
  --glow-color: rgba(249, 115, 22, 0.5);
}

.state-dead .pet-container {
  --glow-color: rgba(107, 114, 128, 0.2);
}
```

**Step 2: 添加光晕动画**

修改 `src/components/PetWidget.vue`，添加光晕脉冲动画:

```css
@keyframes glow-pulse {
  0%, 100% { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 0 20px var(--glow-color); }
  50% { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 0 30px var(--glow-color); }
}

.pet-container {
  animation: glow-pulse 2s ease-in-out infinite;
}
```

**Step 3: 提交光晕效果**

运行:
```bash
git add src/components/PetWidget.vue
git commit -m "feat: add neon glow effect for pet widget"
```

---

## 任务 13：macOS 托盘基础支持

**文件：**
- 创建: `src-tauri/src/tray.rs`
- 修改: `src-tauri/src/main.rs`

**Step 1: 创建托盘模块**

创建 `src-tauri/src/tray.rs`:

```rust
use tauri::{AppHandle, Manager, TrayIcon, TrayIconEvent};
use crate::events::UsageData;

/// 创建系统托盘图标（仅 macOS）
#[cfg(target_os = "macos")]
pub fn create_system_tray(app: &AppHandle) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    use tauri::image::Image;

    // 使用默认图标（后续替换为状态图标）
    let icon = Image::from_bytes(include_bytes!("../icons/default.png"))?;

    let tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("PlanGuard")
        .on_tray_icon_event(|app, event| {
            match event {
                TrayIconEvent::Click { .. } => {
                    // 显示气泡窗口（后续实现）
                    println!("Tray icon clicked");
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(tray)
}

/// 更新托盘图标状态（仅 macOS）
#[cfg(target_os = "macos")]
pub fn update_tray_icon(tray: &TrayIcon, state: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::image::Image;

    let icon_path = match state {
        "Fresh" => "icons/fresh.png",
        "Flow" => "icons/flow.png",
        "Warning" => "icons/warning.png",
        "Panic" => "icons/panic.png",
        "Dead" => "icons/dead.png",
        _ => "icons/default.png",
    };

    let icon = Image::from_path(icon_path)?;
    tray.set_icon(Some(icon))?;

    Ok(())
}

/// Windows 平台空实现
#[cfg(not(target_os = "macos"))]
pub fn create_system_tray(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn update_tray_icon(_tray: &(), _state: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
```

**Step 2: 简化托盘模块（使用占位符）**

修改 `src-tauri/src/tray.rs`:

```rust
use tauri::AppHandle;

/// 创建系统托盘（仅 macOS，Windows 空实现）
#[cfg(target_os = "macos")]
pub fn create_system_tray(app: &AppHandle) -> Result<(), String> {
    // macOS 托盘实现（后续添加图标）
    println!("macOS system tray created");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn create_system_tray(_app: &AppHandle) -> Result<(), String> {
    Ok(())
}
```

**Step 3: 在 main.rs 中声明并初始化托盘**

修改 `src-tauri/src/main.rs`:

```rust
mod events;
mod polling;
mod commands;
mod tray;

use events::UsageData;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage
        ])
        .setup(|app| {
            // macOS 创建系统托盘
            #[cfg(target_os = "macos")]
            {
                if let Err(e) = tray::create_system_tray(app.handle()) {
                    eprintln!("Failed to create tray: {}", e);
                }
            }

            // 启动轮询服务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = polling::start_polling_loop(app_handle).await {
                    eprintln!("Polling loop error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
```

**Step 4: 验证编译**

运行: `cd src-tauri && cargo check`

预期: 编译成功

**Step 5: 提交托盘支持**

运行:
```bash
git add src-tauri/src/tray.rs src-tauri/src/main.rs
git commit -m "feat: add macOS system tray support (placeholder)"
```

---

## 任务 14：创建 macOS 气泡面板组件

**文件：**
- 创建: `src/components/PopoverPanel.vue`

**Step 1: 创建气泡面板组件**

创建 `src/components/PopoverPanel.vue`:

```vue
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { usagePercent, petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const isVisible = ref(false)

const toggle = () => {
  isVisible.value = !isVisible.value
}

defineExpose({
  toggle
})

onMounted(async () => {
  await setupEventListener()
})
</script>

<template>
  <div v-if="isVisible" class="popover-panel">
    <div class="popover-header">
      <h3>PlanGuard</h3>
    </div>

    <div class="popover-content">
      <div class="pet-display" :style="{ backgroundColor: stateColor }">
        <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
          <div class="eye left"></div>
          <div class="eye right"></div>
          <div class="mouth"></div>
        </div>
      </div>

      <div class="usage-info">
        <div class="usage-label">使用量</div>
        <div class="usage-value">{{ usageData.used }} / {{ usageData.total }}</div>
        <div class="usage-percent" :style="{ color: stateColor }">
          {{ Math.round(usagePercent) }}%
        </div>
      </div>

      <div class="progress-bar">
        <div class="progress-track">
          <div class="progress-fill" :style="{ width: `${usagePercent}%`, backgroundColor: stateColor }"></div>
        </div>
      </div>

      <div class="state-badge" :style="{ backgroundColor: stateColor }">
        {{ petState }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.popover-panel {
  width: 280px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
  overflow: hidden;
}

.popover-header {
  padding: 16px;
  background: #f5f5f5;
  border-bottom: 1px solid #e5e5e5;
}

.popover-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.popover-content {
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.pet-display {
  width: 80px;
  height: 80px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.5s ease;
}

.pet-face {
  position: relative;
  width: 50px;
  height: 40px;
}

.eye {
  position: absolute;
  width: 8px;
  height: 8px;
  background: white;
  border-radius: 50%;
  top: 10px;
}

.eye.left { left: 8px; }
.eye.right { right: 8px; }

.mouth {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  width: 16px;
  height: 8px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 16px 16px;
}

.usage-info {
  text-align: center;
}

.usage-label {
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.usage-value {
  font-size: 24px;
  font-weight: 700;
  color: #333;
}

.usage-percent {
  font-size: 32px;
  font-weight: 800;
}

.progress-bar {
  width: 100%;
}

.progress-track {
  width: 100%;
  height: 8px;
  background: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  transition: width 0.5s ease, background-color 0.5s ease;
}

.state-badge {
  padding: 6px 16px;
  border-radius: 20px;
  color: white;
  font-size: 14px;
  font-weight: 600;
}
</style>
```

**Step 2: 提交气泡面板组件**

运行:
```bash
git add src/components/PopoverPanel.vue
git commit -m "feat: add PopoverPanel component for macOS"
```

---

## 任务 15：整合所有组件

**文件：**
- 修改: `src/App.vue`
- 修改: `src/style.css`

**Step 1: 更新 App.vue 整合组件**

修改 `src/App.vue`:

```vue
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import PetWidget from './components/PetWidget.vue'
import PopoverPanel from './components/PopoverPanel.vue'
import { useUsageState } from './composables/useUsageState'
import { useTauriEvents } from './composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const popoverRef = ref<InstanceType<typeof PopoverPanel> | null>(null)

// 检测平台
const isWindows = ref(false)
const isMacOS = ref(false)

onMounted(async () => {
  // 简单的平台检测
  if (navigator.userAgent.includes('Windows')) {
    isWindows.value = true
  } else if (navigator.userAgent.includes('Mac')) {
    isMacOS.value = true
  }

  await setupEventListener()
})
</script>

<template>
  <div class="app-container">
    <!-- Windows: 悬浮窗模式 -->
    <PetWidget v-if="isWindows" />

    <!-- macOS: 托盘 + 气泡面板模式 -->
    <div v-else-if="isMacOS" class="macos-container">
      <p class="hint">点击菜单栏图标查看状态</p>
      <PopoverPanel ref="popoverRef" />
    </div>

    <!-- 其他平台 -->
    <div v-else class="unknown-platform">
      <p>不支持的平台</p>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
}

.macos-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  text-align: center;
}

.hint {
  color: #666;
  font-size: 14px;
}

.unknown-platform {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}
</style>
```

**Step 2: 提交整合更新**

运行:
```bash
git add src/App.vue
git commit -m "feat: integrate all components with platform detection"
```

---

## 任务 16：添加错误处理和边界情况

**文件：**
- 修改: `src-tauri/src/polling.rs`
- 修改: `src/composables/useTauriEvents.ts`

**Step 1: 改进轮询错误处理**

修改 `src-tauri/src/polling.rs`:

```rust
use crate::events::{UsageData, EVENT_USAGE_UPDATE};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

/// Mock 数据 - 返回固定的使用量数据
async fn fetch_usage() -> Result<UsageData, String> {
    // 模拟 API 请求延迟
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Mock 数据：65% 使用量
    Ok(UsageData {
        used: 65,
        total: 100,
    })
}

/// 启动轮询循环
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut timer = interval(Duration::from_secs(300)); // 5 分钟

    loop {
        timer.tick().await;

        match fetch_usage().await {
            Ok(data) => {
                if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                    eprintln!("Failed to emit usage update: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch usage: {}", e);
                // 发送错误事件到前端
                let _ = app.emit("usage-error", e);
            }
        }
    }
}
```

**Step 2: 添加前端错误处理**

修改 `src/composables/useTauriEvents.ts`:

```typescript
import { ref, computed, type Ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { UsageData } from './useUsageState'

export function useTauriEvents() {
  const usageData: Ref<UsageData> = ref({ used: 0, total: 100 })
  const error: Ref<string | null> = ref(null)
  const isLoading = ref(false)

  // 监听 Rust 端推送的 usage-update 事件
  const setupEventListener = async () => {
    // 监听数据更新
    const unlistenUpdate = await listen<UsageData>('usage-update', (event) => {
      usageData.value = event.payload
      error.value = null
      isLoading.value = false
    })

    // 监听错误
    const unlistenError = await listen<string>('usage-error', (event) => {
      error.value = event.payload
      isLoading.value = false
    })

    // 返回清理函数
    return () => {
      unlistenUpdate()
      unlistenError()
    }
  }

  return {
    usageData,
    error,
    isLoading,
    setupEventListener
  }
}
```

**Step 3: 提交错误处理**

运行:
```bash
git add src-tauri/src/polling.rs src/composables/useTauriEvents.ts
git commit -m "feat: add error handling for polling and events"
```

---

## 任务 17：最终测试和验证

**文件：**
- 无修改

**Step 1: 运行完整测试**

运行: `npm run tauri:dev`

预期:
- 应用启动成功
- Windows 下显示悬浮窗，可拖拽
- 悬浮时显示进度条
- 宠物颜色和动画根据 Mock 数据（65%）显示为 Warning 状态

**Step 2: 验证状态切换**

临时修改 Mock 数据测试不同状态:

修改 `src-tauri/src/polling.rs`:

```rust
// 测试 Fresh 状态
Ok(UsageData { used: 20, total: 100 })

// 测试 Flow 状态
Ok(UsageData { used: 50, total: 100 })

// 测试 Warning 状态
Ok(UsageData { used: 70, total: 100 })

// 测试 Panic 状态
Ok(UsageData { used: 90, total: 100 })

// 测试 Dead 状态
Ok(UsageData { used: 100, total: 100 })
```

预期: 每种状态显示正确的颜色和动画

**Step 3: 恢复默认 Mock 数据**

修改 `src-tauri/src/polling.rs`:

```rust
Ok(UsageData {
    used: 65,
    total: 100,
})
```

**Step 4: 提交最终版本**

运行:
```bash
git add .
git commit -m "feat: complete PlanGuard implementation"
```

---

## 任务 18：添加 README 文档

**文件：**
- 创建: `README.md`

**Step 1: 创建项目文档**

创建 `README.md`:

```markdown
# PlanGuard

PlanGuard 是一个跨平台桌面小工具，以"像素风电子宠物"形态展示 codingplan 的使用量。

## 功能

- 实时监控 API 使用量
- 根据用量百分比展示不同状态的宠物
- Windows：悬浮窗 + 拖拽支持
- macOS：系统托盘 + 气泡面板（开发中）

## 状态说明

| 状态 | 百分比 | 颜色 | 动画 |
|------|--------|------|------|
| Fresh | 0-30% | 绿色 | 呼吸 |
| Flow | 31-60% | 蓝色 | 敲打 |
| Warning | 61-80% | 黄色 | 抖动 |
| Panic | 81-95% | 橙色 | 冒汗 |
| Dead | 96-100% | 灰色 | 宕机 |

## 开发

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri:dev

# 构建
npm run tauri:build
```

## 技术栈

- Tauri v2 (Rust)
- Vue 3 + TypeScript
- Tailwind CSS

## 许可

MIT
```

**Step 2: 提交文档**

运行:
```bash
git add README.md
git commit -m "docs: add README documentation"
```

---

## 总结

完成以上所有任务后，PlanGuard 项目将具备：

1. ✅ Tauri + Vue 3 项目结构
2. ✅ 事件驱动架构（Rust 推送数据，Vue 监听更新）
3. ✅ 5 种状态的状态机和动画系统
4. ✅ Windows 悬浮窗（透明、可拖拽、悬浮展开进度条）
5. ✅ macOS 托盘基础支持（气泡面板组件已准备）
6. ✅ Mock 数据轮询（5 分钟间隔）
7. ✅ 错误处理机制
8. ✅ 跨平台代码结构

### 后续扩展方向

- 替换占位符为真实雪碧图
- 集成真实 API
- 添加配置文件支持
- 完善 macOS 托盘图标切换
- 添加手动刷新按钮
- 添加通知功能
