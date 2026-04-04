# 悬停扩展窗口实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 实现鼠标悬停时窗口从 96×96px 扩展到 246×246px，离开时收缩，同时静默刷新接口数据。

**Architecture:** 使用 Tauri 的 `window.setSize()` API 改变窗口尺寸，配合 CSS transition 实现平滑过渡动画。通过防抖机制避免频繁的接口调用。

**Tech Stack:** Vue 3 Composition API, Tauri Window API, CSS transitions

---

## Task 1: 添加常量和状态定义

**Files:**
- Modify: `src/components/PetWidget.vue:1-30`

**Step 1: 添加常量定义**

在 `<script setup>` 中现有的导入语句后，添加以下常量：

```typescript
// 窗口尺寸常量
const WINDOW_SIZE_SMALL = 96
const WINDOW_SIZE_LARGE = 246
const TRANSITION_DURATION_MS = 350
const REFRESH_DEBOUNCE_MS = 500
```

**Step 2: 添加响应式状态**

在 `const isRefreshing = ref(false)` 之后，添加以下状态：

```typescript
// 窗口扩展状态
const isHovered = ref(false)
const isExpanded = ref(false)
const refreshDebounce = ref<number | null>(null)
const isTransitioning = ref(false)
```

**Step 3: 保存文件并验证语法**

运行: `npm run type-check 2>&1 | head -20`
预期: 无 TypeScript 语法错误

**Step 4: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: add constants and state for hover expand feature"
```

---

## Task 2: 实现窗口扩展方法

**Files:**
- Modify: `src/components/PetWidget.vue:50-70`

**Step 1: 实现 expandWindow 方法**

在 `startDrag` 函数之后，添加以下方法：

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
```

**Step 2: 保存文件**

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement expandWindow method"
```

---

## Task 3: 实现窗口收缩方法

**Files:**
- Modify: `src/components/PetWidget.vue:70-90`

**Step 1: 实现 collapseWindow 方法**

在 `expandWindow` 函数之后，添加以下方法：

```typescript
// 窗口收缩
async function collapseWindow() {
  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setSize({ width: WINDOW_SIZE_SMALL, height: WINDOW_SIZE_SMALL })
    isExpanded.value = false
  } catch (error) {
    console.error('Failed to collapse window:', error)
    // 强制重置状态
    isExpanded.value = false
  }
}
```

**Step 2: 保存文件**

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement collapseWindow method"
```

---

## Task 4: 实现静默刷新方法

**Files:**
- Modify: `src/components/PetWidget.vue:90-120`

**Step 1: 实现静默刷新方法**

在 `collapseWindow` 函数之后，添加以下方法：

```typescript
// 静默刷新数据（不显示加载提示）
async function refreshUsageData() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const data = await invoke<typeof usageData.value>('get_current_usage')
    usageData.value = data
    const now = new Date()
    lastUpdateTime.value = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`
    fetchError.value = ''
  } catch (err) {
    fetchError.value = String(err)
    console.error('Silent refresh failed:', err)
  }
}
```

**Step 2: 保存文件**

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement silent refresh method"
```

---

## Task 5: 修改悬停刷新逻辑

**Files:**
- Modify: `src/components/PetWidget.vue:32-48`

**Step 1: 替换现有的 onHoverRefresh 函数**

将现有的 `onHoverRefresh` 函数替换为：

```typescript
// 鼠标悬停时扩展窗口并刷新接口
async function onHoverRefresh() {
  // 防抖检查
  if (refreshDebounce.value) return
  
  refreshDebounce.value = window.setTimeout(() => {
    refreshDebounce.value = null
  }, REFRESH_DEBOUNCE_MS)
  
  // 扩展窗口
  if (!isExpanded.value) {
    await expandWindow()
  }
  
  // 静默刷新接口
  await refreshUsageData()
}
```

**Step 2: 保存文件**

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: update hover refresh logic with debounce and window expand"
```

---

## Task 6: 实现悬停离开处理

**Files:**
- Modify: `src/components/PetWidget.vue:120-135`

**Step 1: 实现 onHoverLeave 方法**

在 `refreshUsageData` 函数之后，添加以下方法：

```typescript
// 鼠标离开时收缩窗口
async function onHoverLeave() {
  // 清除防抖定时器
  if (refreshDebounce.value) {
    clearTimeout(refreshDebounce.value)
    refreshDebounce.value = null
  }
  
  // 收缩窗口
  await collapseWindow()
}
```

**Step 2: 在 onMounted 中添加清理逻辑**

修改 `onMounted` 函数，在最后添加返回清理函数：

```typescript
onMounted(async () => {
  await setupEventListener()
  
  // 组件卸载时清理
  return () => {
    if (refreshDebounce.value) {
      clearTimeout(refreshDebounce.value)
    }
  }
})
```

**Step 3: 保存文件**

**Step 4: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement hover leave handler with cleanup"
```

---

## Task 7: 更新模板事件绑定

**Files:**
- Modify: `src/components/PetWidget.vue:67-71`

**Step 1: 添加 @mouseleave 事件**

在模板中找到 `.pet-widget` 元素，添加 `@mouseleave` 事件：

```vue
<div class="pet-widget" :class="`pet-${petState.toLowerCase()}`"
  data-tauri-drag-region
  @mousedown="startDrag"
  @mouseenter="onHoverRefresh"
  @mouseleave="onHoverLeave"
  :class="{ expanded: isExpanded }"
>
```

**Step 2: 保存文件**

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: bind mouseleave event and expanded class"
```

---

## Task 8: 更新 CSS 样式

**Files:**
- Modify: `src/components/PetWidget.vue:347-365`

**Step 1: 更新 .pet-widget 基础样式**

修改 `.pet-widget` 样式，添加过渡动画：

```css
.pet-widget {
  width: 96px;
  height: 96px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent !important;
  position: relative;
  cursor: pointer;
  user-select: none;
  pointer-events: auto;
  -webkit-app-region: drag;
  app-region: drag;
  transition: width 350ms ease-out, height 350ms ease-out;
}
.pet-widget:active { cursor: pointer; }
```

**Step 2: 更新 .heart-msg 样式**

修改 `.heart-msg` 样式，使其自适应窗口大小：

```css
.heart-msg {
  position: absolute;
  inset: 3px;
  border-radius: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  background: rgba(8, 14, 28, 0.91);
  color: #E2E8F0;
  pointer-events: none;
  opacity: 0;
  transition: all 350ms ease-out;
  z-index: 10;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
  border: 1px solid rgba(255,255,255,0.1);
  overflow: hidden;
  padding: 6px 9px;
}
.pet-widget:hover .heart-msg,
.pet-widget.expanded .heart-msg { 
  opacity: 1; 
}
```

**Step 3: 添加扩展状态下的内容放大样式**

在 `.heart-msg` 样式之后，添加：

```css
.pet-widget.expanded .heart-msg {
  transform: scale(1.5);
}
```

**Step 4: 保存文件**

**Step 5: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: add CSS transitions for window expansion"
```

---

## Task 9: 测试悬停扩展功能

**Files:**
- Test: `src/components/PetWidget.vue`

**Step 1: 启动开发服务器**

运行: `npm run tauri:dev`
预期: 应用正常启动，显示 96×96px 窗口

**Step 2: 测试悬停扩展**

操作: 将鼠标移入图标
预期:
- 窗口平滑扩展到 246×246px
- 过渡动画时长约 350ms
- 信息面板显示

**Step 3: 测试离开收缩**

操作: 将鼠标移出窗口
预期:
- 窗口平滑收缩回 96×96px
- 信息面板隐藏

**Step 4: 测试快速进出**

操作: 快速将鼠标移入移出多次
预期:
- 不会出现闪烁
- 动画流畅

**Step 5: 测试拖动兼容**

操作: 在窗口扩展状态下拖动窗口
预期:
- 拖动功能正常

**Step 6: 记录测试结果**

如有问题，记录并修复。如全部通过，继续下一步。

---

## Task 10: 测试防抖机制

**Files:**
- Test: `src/components/PetWidget.vue`

**Step 1: 打开开发者工具**

在运行的应用中，按 F12 打开开发者工具

**Step 2: 监控接口调用**

在 Console 中，添加以下日志监控：

```javascript
// 在 onHoverRefresh 中添加临时日志
console.log('Hover refresh triggered at:', new Date().toISOString())
```

**Step 3: 测试防抖**

操作: 在 500ms 内多次悬停
预期: 只触发一次接口调用

**Step 4: 测试超过防抖时间**

操作: 等待超过 500ms 后再次悬停
预期: 触发新的接口调用

**Step 5: 移除临时日志**

**Step 6: 提交修复（如有）**

```bash
git add src/components/PetWidget.vue
git commit -m "fix: adjust debounce timing if needed"
```

---

## Task 11: 测试错误处理

**Files:**
- Test: `src/components/PetWidget.vue`

**Step 1: 测试接口失败场景**

操作: 修改 mock 数据使其返回错误
预期: 显示错误提示，窗口状态正常

**Step 2: 测试窗口尺寸调整失败**

操作: 在浏览器控制台模拟 Tauri API 失败
预期: 捕获异常，记录日志，状态回退

**Step 3: 验证错误日志**

操作: 检查控制台错误日志
预期: 有清晰的错误信息

**Step 4: 提交修复（如有）**

```bash
git add src/components/PetWidget.vue
git commit -m "fix: improve error handling"
```

---

## Task 12: 最终验收测试

**Files:**
- Test: 完整功能测试

**Step 1: 完整功能测试清单**

- [ ] 悬停扩展到 246×246px
- [ ] 离开收缩到 96×96px
- [ ] 动画流畅，约 350ms
- [ ] 防抖机制工作正常
- [ ] 拖动功能不受影响
- [ ] 接口刷新正常
- [ ] 错误处理正确

**Step 2: 性能检查**

- [ ] 动画帧率保持 60fps
- [ ] 无明显 CPU 占用增加
- [ ] 内存占用稳定

**Step 3: 跨平台测试（如适用）**

- [ ] Windows 测试通过
- [ ] macOS 测试通过（如有）

**Step 4: 最终提交**

```bash
git add .
git commit -m "feat: complete hover expand window feature

- Implement window expansion from 96x96 to 246x246 on hover
- Add smooth CSS transitions (350ms)
- Add debounce mechanism (500ms) for API calls
- Add error handling for window resize failures
- Maintain drag functionality compatibility
"
```

---

## 实施检查清单

- [ ] Task 1: 添加常量和状态定义
- [ ] Task 2: 实现窗口扩展方法
- [ ] Task 3: 实现窗口收缩方法
- [ ] Task 4: 实现静默刷新方法
- [ ] Task 5: 修改悬停刷新逻辑
- [ ] Task 6: 实现悬停离开处理
- [ ] Task 7: 更新模板事件绑定
- [ ] Task 8: 更新 CSS 样式
- [ ] Task 9: 测试悬停扩展功能
- [ ] Task 10: 测试防抖机制
- [ ] Task 11: 测试错误处理
- [ ] Task 12: 最终验收测试

---

## 相关文档

- 设计文档: [docs/plans/2026-04-04-hover-expand-window-design.md](docs/plans/2026-04-04-hover-expand-window-design.md)
- 项目说明: [CLAUDE.md](CLAUDE.md)
- 组件文件: [src/components/PetWidget.vue](src/components/PetWidget.vue)
