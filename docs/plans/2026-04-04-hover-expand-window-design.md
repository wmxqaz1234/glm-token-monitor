# 悬停扩展窗口设计文档

**日期**: 2026-04-04
**功能**: 鼠标悬停时窗口扩展并刷新数据
**方案**: CSS Transform + Tauri setSize

---

## 1. 需求概述

### 功能需求

| 场景 | 行为 |
|------|------|
| 鼠标悬停 | 窗口从 96×96px 扩展到 246×246px，静默刷新接口 |
| 鼠标离开 | 窗口收缩回 96×96px，信息面板隐藏 |

### 非功能需求

- 动画时长：350ms 缓动平滑过渡
- 防抖机制：500ms 内只触发一次刷新
- 错误处理：接口失败不影响窗口状态

---

## 2. 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri 窗口层                          │
│  ┌───────────────────────────────────────────────────┐  │
│  │   PetWidget.vue (96x96 → 246x246)                 │  │
│  │   ┌─────────────────────────────────────────┐     │  │
│  │   │  .pet-widget 容器                       │     │  │
│  │   │  ┌─────────────┐  ┌─────────────────┐  │     │  │
│  │   │  │ SVG 宠物图标 │  │ .heart-msg     │  │     │  │
│  │   │  │ (80x80)     │  │   信息面板      │  │     │  │
│  │   │  └─────────────┘  │   (悬停显示)     │  │     │  │
│  │   │                   └─────────────────┘  │     │  │
│  │   └─────────────────────────────────────────┘     │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 状态管理

| 状态 | 类型 | 含义 |
|------|------|------|
| `isHovered` | `Ref<boolean>` | 是否处于悬停状态 |
| `isExpanded` | `Ref<boolean>` | 窗口是否已扩展 |
| `lastRefreshTime` | `Ref<string>` | 上次刷新时间 |
| `refreshDebounce` | `Ref<number \| null>` | 防抖定时器 ID |
| `isTransitioning` | `Ref<boolean>` | 是否正在过渡中 |

### 尺寸规格

| 状态 | 窗口尺寸 | 信息面板尺寸 | 过渡时间 |
|------|----------|--------------|----------|
| 默认 | 96×96px | 90×90px（圆形） | 350ms |
| 悬停 | 246×246px | 240×240px（圆形） | 350ms |

---

## 3. 组件设计

### PetWidget.vue 改动

**新增常量：**

```typescript
const REFRESH_DEBOUNCE_MS = 500
const WINDOW_SIZE_SMALL = 96
const WINDOW_SIZE_LARGE = 246
const TRANSITION_DURATION_MS = 350
```

**新增状态：**

```typescript
const isHovered = ref(false)
const isExpanded = ref(false)
const refreshDebounce = ref<number | null>(null)
const isTransitioning = ref(false)
```

**核心方法：**

```typescript
// 窗口扩展
async function expandWindow() {
  if (isTransitioning.value) return
  isTransitioning.value = true
  
  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setSize({ width: WINDOW_SIZE_LARGE, height: WINDOW_SIZE_LARGE })
    isExpanded.value = true
  } catch (error) {
    console.error('Failed to expand window:', error)
  } finally {
    setTimeout(() => {
      isTransitioning.value = false
    }, TRANSITION_DURATION_MS)
  }
}

// 窗口收缩
async function collapseWindow() {
  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setSize({ width: WINDOW_SIZE_SMALL, height: WINDOW_SIZE_SMALL })
    isExpanded.value = false
  } catch (error) {
    console.error('Failed to collapse window:', error)
    isExpanded.value = false
  }
}

// 修改后的悬停刷新函数
async function onHoverRefresh() {
  if (refreshDebounce.value) return
  
  refreshDebounce.value = window.setTimeout(() => {
    refreshDebounce.value = null
  }, REFRESH_DEBOUNCE_MS)
  
  if (!isExpanded.value) {
    await expandWindow()
  }
  
  await refreshUsageData()
}

// 悬停离开处理
async function onHoverLeave() {
  if (refreshDebounce.value) {
    clearTimeout(refreshDebounce.value)
    refreshDebounce.value = null
  }
  await collapseWindow()
}
```

### CSS 改动

```css
/* 容器过渡动画 */
.pet-widget {
  transition: width 350ms ease-out, height 350ms ease-out;
}

/* 信息面板根据窗口大小自适应 */
.pet-widget .heart-msg {
  width: calc(100% - 6px);
  height: calc(100% - 6px);
  transition: all 350ms ease-out;
}

/* 扩展状态下的内容放大 */
.pet-widget.expanded .heart-msg {
  transform: scale(1.5);
}

/* 移除悬停时的 opacity 过渡（改用窗口扩展） */
.pet-widget:hover .heart-msg {
  opacity: 1;
}
```

---

## 4. 数据流

### 事件流程

```
鼠标悬停 → onHoverRefresh()
    ├─ 防抖检查（500ms）
    ├─ expandWindow() → Tauri setSize(246, 246)
    └─ refreshUsageData() → invoke('get_current_usage')

鼠标离开 → onHoverLeave()
    ├─ 清除防抖定时器
    └─ collapseWindow() → Tauri setSize(96, 96)
```

### 错误处理

| 场景 | 处理方式 |
|------|----------|
| 窗口尺寸调整失败 | 捕获异常并回退状态 |
| 接口请求失败 | 显示错误信息但不影响窗口状态 |
| 快速进出悬停 | 防抖 + 状态锁 |
| 窗口被拖动中悬停 | 正常响应，拖动和悬停独立处理 |

---

## 5. 测试计划

### 功能测试

| 测试用例 | 测试步骤 | 预期结果 |
|----------|----------|----------|
| 悬停扩展 | 鼠标移入图标 | 窗口扩展到 246×246px |
| 离开收缩 | 鼠标移出窗口 | 窗口收缩到 96×96px |
| 接口刷新 | 悬停时触发 | 调用接口并更新数据 |
| 防抖机制 | 快速进出悬停 | 500ms 内只触发一次 |
| 拖动兼容 | 拖动时悬停 | 拖动功能正常 |

### 边界测试

| 测试用例 | 测试步骤 | 预期结果 |
|----------|----------|----------|
| 快速进出 | 鼠标快速进出 | 不出现闪烁 |
| 接口失败 | 模拟接口错误 | 显示错误提示 |
| 窗口调整失败 | 模拟 API 失败 | 状态回退 |

---

## 6. 实施检查清单

- [ ] 在 PetWidget.vue 中添加新状态和常量
- [ ] 实现 expandWindow() 方法
- [ ] 实现 collapseWindow() 方法
- [ ] 修改 onHoverRefresh() 方法
- [ ] 添加 onHoverLeave() 方法
- [ ] 更新模板，添加 @mouseleave 事件
- [ ] 更新 CSS 样式
- [ ] 添加组件卸载时的清理逻辑
- [ ] 测试悬停扩展功能
- [ ] 测试离开收缩功能
- [ ] 测试防抖机制
- [ ] 测试错误处理
