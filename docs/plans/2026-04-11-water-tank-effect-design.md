# 宠物储水罐效果设计

**日期**: 2026-04-11
**状态**: 已批准

## 概述

为 PixelGhost 和 JellySpirit 宠物组件添加"储水罐"效果。宠物本身大小不变，但内部颜色填充区域根据用量百分比从下往上显示水位效果。

## 设计目标

- 宠物作为"储水罐"，用量百分比代表"剩余水量"
- 0% 用量 → 水位满（储水罐满）
- 100% 用量 → 水位空（储水罐空）
- 有水区域显示宠物原色，无水区域显示半透明浅色

## 技术方案：动态 Path 重绘

### 核心原理

根据水位百分比动态生成新的 SVG 路径，将宠物形状分割为"有水区域"和"无水区域"两部分分别渲染。

### 水位计算公式

```typescript
// 填充比例：0% 用量 = 100% 填充，100% 用量 = 0% 填充
const fillRatio = 1 - (usagePercent / 100)

// 水位线 Y 坐标（viewBox 坐标系，0在顶部）
const waterY = viewBoxBottom - (viewBoxHeight * fillRatio)
```

**示例**：
- 0% 用量 → fillRatio = 1.0 → 水位线在顶部
- 50% 用量 → fillRatio = 0.5 → 水位线在中间
- 100% 用量 → fillRatio = 0.0 → 水位线在底部

### 组件 Props

```typescript
interface Props {
  // ... 现有 props (color, strokeColor, eyeColor, etc.)
  waterLevel?: number      // 水位百分比 (0-100)
  emptyColor?: string      // 空罐颜色，默认 'rgba(229, 231, 235, 0.5)'
}
```

## 实现细节

### PixelGhost（幽灵）

**原始路径分析**：
- viewBox: "0 0 64 64"
- 身体路径：从顶部开始，波浪形底部
- 底部波浪：通过多个贝塞尔曲线形成

**动态路径生成**：
1. 解析原始路径，找到水位线的切入/切出点
2. 生成"有水部分"路径：从左侧切入点 → 底部波浪 → 右侧切出点 → 水平线回到左侧
3. 生成"无水部分"路径：原始顶部 → 左侧切出点 → 水平线到右侧切入点 → 原始底部

**数学计算**：
- 水位线与路径的水平线段相交：直接计算交点
- 水位线与贝塞尔曲线相交：使用数值方法求解

### JellySpirit（果冻）

**原始形状分析**：
- viewBox: "0 0 64 64"
- 身体：椭圆 `<ellipse cx="32" cy="36" rx="24" ry="26">`
- 中心点 (32, 36)，水平半径 24，垂直半径 26

**动态路径生成**：
1. 椭圆方程：`(x-32)²/24² + (y-36)²/26² = 1`
2. 水位线 y = waterY
3. 求解交点 x 坐标：
   ```
   x = 32 ± 24 × sqrt(1 - (waterY-36)²/26²)
   ```
4. 生成"有水部分"路径：从左交点 → 椭圆底部弧线 → 右交点 → 水平线闭合
5. 生成"无水部分"路径：椭圆顶部弧线 → 左交点 → 水平线到右交点 → 闭合

### 颜色应用

```vue
<!-- 无水部分（上层） -->
<path :fill="emptyColor" d="[无水路径]" />

<!-- 有水部分（下层） -->
<path :fill="color" d="[有水路径]" />
```

## 数据流

```
usageData { used, total }
    ↓
useUsageState.usagePercent
    ↓
waterLevel = 100 - usagePercent  // 0% 用量 = 100% 水位
    ↓
PetWidget 传递 :water-level prop
    ↓
宠物组件计算动态路径
    ↓
渲染两层路径（无水 + 有水）
```

## 动画效果

### 平滑过渡

使用 Vue 的 `<transition>` 或 CSS transition 实现水位变化的平滑动画：

```vue
<svg>
  <g class="water-group">
    <path :fill="emptyColor" :d="emptyPath" />
    <path :fill="color" :d="waterPath" class="water-fill" />
  </g>
</svg>

<style>
.water-fill {
  transition: d 0.3s ease-out;
}
</style>
```

### 呼吸效果保留

原有的状态动画（呼吸、摇摆、抖动）保留在 `<g>` 容器上，不影响水位效果。

## 边界情况处理

### 水位 = 0（完全空）
- 只渲染"无水部分"，"有水部分"不渲染
- 或水位线设置在 viewBox 外部

### 水位 = 100（完全满）
- 只渲染"有水部分"，"无水部分"不渲染
- 效果等同于原始宠物

### 椭圆完全在水下或水上
- JellySpirit 当水位线高于或低于椭圆时，使用完整椭圆路径

## 性能优化

### 路径缓存
- 对常用水位值（0, 25, 50, 75, 100）预计算路径
- 使用 Map 缓存计算结果

### 计算节流
- 水位变化时使用防抖（debounce）避免频繁重计算
- 100ms 延迟确保流畅体验

### SVG 优化
- 使用 `<path>` 而非多个形状拼接
- 减少 DOM 节点数量

## 参考文档

- SVG Path 命令参考：https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
- 贝塞尔曲线数学：https://en.wikipedia.org/wiki/B%C3%A9zier_curve
- 椭圆方程：https://en.wikipedia.org/wiki/Ellipse
