# 宠物颜色实时渐变实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 为 PixelGhost 和 JellySpirit 宠物组件添加基于用量百分比的实时 HSL 颜色渐变效果

**架构:** 在 `useUsageState.ts` 中添加 HSL 颜色计算逻辑，宠物组件接收动态颜色 prop，移除固定状态颜色配置

**技术栈:** Vue 3 Composition API, TypeScript, HSL 颜色模型, SVG

---

## Task 1: 在 useUsageState.ts 中添加渐变颜色计算

**文件:**
- Modify: `src/composables/useUsageState.ts`

**Step 1: 添加渐变颜色计算属性**

在 `useUsageState` 函数中添加两个新的计算属性：

```typescript
const gradientColor = computed<string>(() => {
  const p = usagePercent.value
  // Dead 状态：100%及以上使用灰色
  if (p >= 100) return '#6B7280'
  // 0-100% 映射到 HSL 色相 120°(绿) → 0°(红)
  const hue = 120 * (1 - p / 100)
  return `hsl(${hue}, 75%, 45%)`
})

const gradientStrokeColor = computed<string>(() => {
  const p = usagePercent.value
  if (p >= 100) return '#4B5563'
  const hue = 120 * (1 - p / 100)
  return `hsl(${hue}, 80%, 35%)`
})
```

**Step 2: 更新返回值**

在 return 对象中添加新属性：

```typescript
return {
  usagePercent,
  petState,
  stateColor,
  gradientColor,      // 新增
  gradientStrokeColor // 新增
}
```

**Step 3: 验证语法**

运行: `npm run type-check` 或 `npx vue-tsc --noEmit`
预期: 无类型错误

**Step 4: 提交**

```bash
git add src/composables/useUsageState.ts
git commit -m "feat(composable): add gradient color calculation based on usage percent"
```

---

## Task 2: 修改 PixelGhost.vue 使用动态颜色

**文件:**
- Modify: `src/components/pets/PixelGhost.vue`

**Step 1: 修改 Props 接口**

将 Props 从接收 `state` 改为接收 `color` 相关属性：

```typescript
interface Props {
  color: string          // 主色
  strokeColor: string    // 描边色
  eyeColor?: string      // 眼睛颜色（可选，默认深色）
  width?: number
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100
})
```

**Step 2: 移除固定颜色配置**

删除原有的 `colors` 对象和 `currentColors` computed：

```typescript
// 删除这些代码：
const colors = {
  Fresh:  { fill: '#10B981', stroke: '#059669', eye: '#1F2937', blush: '#34D399' },
  Flow:   { fill: '#3B82F6', stroke: '#2563EB', eye: '#1F2937', blush: '#60A5FA' },
  // ... 其他状态
}
const currentColors = computed(() => colors[props.state])
```

**Step 3: 更新模板中的颜色引用**

将所有 `currentColors.fill` 替换为 `props.color`，`currentColors.stroke` 替换为 `props.strokeColor`：

```vue
<!-- 幽灵身体 -->
<path :fill="props.color" :stroke="props.strokeColor" ...>

<!-- 腮红 - 需要计算透明度版本 -->
<ellipse :fill="props.color" opacity="0.35" ...>

<!-- 嘴巴 -->
<path :stroke="props.strokeColor" ...>
```

**Step 4: 保留状态驱动的动画**

动画相关的 `v-if="state === ..."` 条件保留，但需要通过其他方式获取 state。添加一个新的 prop：

```typescript
interface Props {
  // ... 其他 props
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'  // 仅用于动画，不用于颜色
}
```

**Step 5: 提交**

```bash
git add src/components/pets/PixelGhost.vue
git commit -m "refactor(pet): PixelGhost use dynamic color props instead of state colors"
```

---

## Task 3: 修改 JellySpirit.vue 使用动态颜色

**文件:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 修改 Props 接口**

```typescript
interface Props {
  color: string
  strokeColor: string
  eyeColor?: string
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'  // 仅用于动画
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100
})
```

**Step 2: 移除固定颜色配置**

删除 `colors` 对象和 `currentColors` computed。

**Step 3: 更新模板颜色引用**

```vue
<!-- 身体 -->
<ellipse :fill="props.color" :stroke="props.strokeColor" ...>

<!-- 眼睛 -->
<circle :fill="props.eyeColor" ...>
```

**Step 4: 提交**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "refactor(pet): JellySpirit use dynamic color props"
```

---

## Task 4: 更新 PetWidget.vue 传递颜色

**文件:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 导入渐变颜色**

从 `useUsageState` 解构新的颜色属性：

```typescript
const { usagePercent, petState, gradientColor, gradientStrokeColor } = useUsageState(used, total)
```

**Step 2: 传递颜色给宠物组件**

更新宠物组件的 props 绑定：

```vue
<PixelGhost
  v-if="currentPet === 'pixel-ghost'"
  :color="gradientColor"
  :stroke-color="gradientStrokeColor"
  :state="petState"
  :width="petSize"
  :height="petSize"
/>

<JellySpirit
  v-else-if="currentPet === 'jelly-spirit'"
  :color="gradientColor"
  :stroke-color="gradientStrokeColor"
  :state="petState"
  :width="petSize"
  :height="petSize"
/>
```

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat(pet): pass gradient colors to pet components"
```

---

## Task 5: 手动测试验证

**Step 1: 启动开发服务器**

运行: `npm run tauri:dev`
预期: 应用正常启动，显示宠物

**Step 2: 测试不同用量下的颜色**

修改 `src-tauri/src/polling.rs` 中的 mock 数据，测试不同百分比：

```rust
// 测试 0% - 绿色
UsageData { used: 0, total: 100, ..default() }

// 测试 30% - 黄绿色
UsageData { used: 30, total: 100, ..default() }

// 测试 60% - 黄色
UsageData { used: 60, total: 100, ..default() }

// 测试 80% - 橙色
UsageData { used: 80, total: 100, ..default() }

// 测试 100% - 红色
UsageData { used: 100, total: 100, ..default() }
```

每次修改后重启应用，验证颜色是否正确。

**Step 3: 验证动画正常**

确认各状态的动画效果（呼吸、摇摆、抖动等）仍然正常工作。

**Step 4: 测试平滑过渡**

如果可能，手动快速修改 mock 数据值（如 30→31→32），观察颜色是否平滑变化。

**Step 5: 记录测试结果**

在项目根目录创建 `test-notes.md` 记录测试结果：

```markdown
# 颜色渐变测试结果

- [ ] 0% 显示绿色
- [ ] 50% 显示黄色
- [ ] 100% 显示红色
- [ ] 动画正常工作
- [ ] 状态切换时颜色正确
```

---

## Task 6: 代码审查和清理

**Step 1: 检查未使用的代码**

搜索项目中是否有其他地方引用了旧的 `state` prop 用于颜色：

运行: `grep -r "currentColors" src/`
预期: 只在 git 历史中，无当前引用

**Step 2: 检查类型定义**

确认 `src/types/config.ts` 中是否有需要更新的类型定义。

**Step 3: 运行完整类型检查**

运行: `npm run type-check` 或 `npx vue-tsc --noEmit`
预期: 无错误

**Step 4: 构建测试**

运行: `npm run tauri:build`
预期: 构建成功

**Step 5: 最终提交**

```bash
git add .
git commit -m "test: verify gradient color implementation works correctly"
```

---

## 验收标准

- [ ] 两个宠物组件都使用动态 HSL 颜色
- [ ] 颜色随用量百分比平滑变化（绿→黄→橙→红）
- [ ] 100%及以上显示灰色
- [ ] 所有动画效果正常工作
- [ ] 类型检查通过
- [ ] 构建成功

## 参考文档

- 设计文档: `docs/plans/2026-04-11-pet-gradient-color-design.md`
- 项目约定: `CLAUDE.md`
- HSL 颜色模型: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hsl
