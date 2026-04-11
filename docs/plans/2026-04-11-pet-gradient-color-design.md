# 宠物颜色实时渐变设计

**日期**: 2026-04-11
**状态**: 已批准

## 概述

为 PixelGhost 和 JellySpirit 两个宠物组件添加基于用量百分比的实时颜色渐变效果，替代原有的5状态离散颜色切换。

## 设计目标

- 实现平滑的颜色过渡，反映实时用量变化
- 保持直观的视觉反馈（绿色=安全，红色=危险）
- 简化组件代码，移除冗余的状态颜色配置

## 技术方案

### 颜色映射：HSL色相连续映射

将用量百分比(0-100%)映射到HSL色相环的120°到0°范围：

```
hue = 120 × (1 - percent/100)
```

**映射结果**：
- 0% → HSL(120, 75%, 45%) - 绿色（Fresh）
- 30% → HSL(84, 75%, 45%) - 黄绿色
- 60% → HSL(48, 75%, 45%) - 黄色
- 80% → HSL(24, 75%, 45%) - 橙色
- 100% → HSL(0, 75%, 45%) - 红色（Dead）

### 修改的文件

#### 1. `src/composables/useUsageState.ts`

添加新的计算属性：

```typescript
const gradientColor = computed<string>(() => {
  const p = usagePercent.value
  const hue = 120 * (1 - p / 100) // 120→0
  return `hsl(${hue}, 75%, 45%)`
})

const gradientStrokeColor = computed<string>(() => {
  const p = usagePercent.value
  const hue = 120 * (1 - p / 100)
  return `hsl(${hue}, 80%, 35%)` // 更深的描边色
})
```

返回值中添加这两个新属性。

#### 2. `src/components/pets/PixelGhost.vue`

**移除**：
- 5状态颜色配置对象

**修改**：
- Props添加 `color: string` 和 `strokeColor: string`
- 直接使用props中的颜色值

#### 3. `src/components/pets/JellySpirit.vue`

同上。

#### 4. `src/components/PetWidget.vue`

传递计算好的颜色给宠物组件。

## 数据流

```
Rust轮询 (5min)
    ↓
UsageData { used, total }
    ↓
useUsageState.usagePercent
    ↓
useUsageState.gradientColor / gradientStrokeColor (HSL计算)
    ↓
PetWidget 传递 color prop
    ↓
PixelGhost/JellySpirit 应用颜色
```

## 动画保留

原有的SVG动画（呼吸、摇摆、眨眼等）完全保留，不受影响。

## 实现要点

1. **Dead状态特殊处理**：当用量≥100%时，仍然使用灰色表示"耗尽"
2. **眼睛颜色**：保持深色不变（`#1F2937`），不跟随渐变
3. **腮红/高光**：基于主色调计算，保持透明度
