# 宠物储水罐效果实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 为 PixelGhost 和 JellySpirit 宠物组件添加储水罐效果，根据用量百分比动态生成 SVG 路径实现水位变化

**架构:** 使用动态 Path 重绘技术，根据水位百分比计算 SVG 路径的切割点，生成"有水区域"和"无水区域"两层路径分别渲染

**技术栈:** Vue 3 Composition API, TypeScript, SVG Path 命令, 数学计算（椭圆方程、贝塞尔曲线交点）

---

## Task 1: 添加水位计算工具函数

**文件:**
- Create: `src/utils/waterLevel.ts`

**Step 1: 创建水位计算工具文件**

创建 `src/utils/waterLevel.ts`，包含水位计算的基础函数：

```typescript
/**
 * 根据用量百分比计算水位填充比例
 * 0% 用量 = 100% 填充（满），100% 用量 = 0% 填充（空）
 */
export function calculateFillRatio(usagePercent: number): number {
  return Math.max(0, Math.min(100, 100 - usagePercent)) / 100
}

/**
 * 计算水位线在 viewBox 中的 Y 坐标
 * @param usagePercent 用量百分比 (0-100)
 * @param viewBoxHeight viewBox 高度
 * @param viewBoxBottom viewBox 底部 Y 坐标
 */
export function calculateWaterY(
  usagePercent: number,
  viewBoxHeight: number,
  viewBoxBottom: number
): number {
  const fillRatio = calculateFillRatio(usagePercent)
  return viewBoxBottom - (viewBoxHeight * fillRatio)
}

/**
 * 计算椭圆与水平线的交点
 * @param waterY 水位线 Y 坐标
 * @param centerX 椭圆中心 X
 * @param centerY 椭圆中心 Y
 * @param radiusX 椭圆水平半径
 * @param radiusY 椭圆垂直半径
 */
export function calculateEllipseIntersection(
  waterY: number,
  centerX: number,
  centerY: number,
  radiusX: number,
  radiusY: number
): { left: number; right: number } | null {
  const dy = waterY - centerY
  const ratioSquared = (dy * dy) / (radiusY * radiusY)

  // 如果水位线在椭圆外，无交点
  if (ratioSquared > 1) {
    return null
  }

  const dx = radiusX * Math.sqrt(1 - ratioSquared)
  return {
    left: centerX - dx,
    right: centerX + dx
  }
}
```

**Step 2: 添加类型定义**

```typescript
export interface WaterPathResult {
  waterPath: string    // 有水区域的 SVG 路径
  emptyPath: string    // 无水区域的 SVG 路径
  isEmpty: boolean     // 是否完全空
  isFull: boolean      // 是否完全满
}
```

**Step 3: 提交**

```bash
git add src/utils/waterLevel.ts
git commit -m "feat(utils): add water level calculation utilities"
```

---

## Task 2: 实现 JellySpirit 椭圆水位路径生成

**文件:**
- Create: `src/utils/jellySpiritWaterPath.ts`

**Step 1: 创建 JellySpirit 水位路径生成器**

创建 `src/utils/jellySpiritWaterPath.ts`：

```typescript
import { calculateWaterY, calculateEllipseIntersection, type WaterPathResult } from './waterLevel'

/**
 * 生成 JellySpirit 的水位路径
 * @param usagePercent 用量百分比 (0-100)
 */
export function generateJellySpiritWaterPath(usagePercent: number): WaterPathResult {
  // JellySpirit viewBox: 0 0 64 64
  const VIEW_BOX = { width: 64, height: 64, bottom: 64 }
  const ELLIPSE = { cx: 32, cy: 36, rx: 24, ry: 26 }

  // 计算水位线 Y 坐标
  const waterY = calculateWaterY(usagePercent, VIEW_BOX.height, VIEW_BOX.bottom)

  // 椭圆顶部和底部 Y 坐标
  const ellipseTop = ELLIPSE.cy - ELLIPSE.ry
  const ellipseBottom = ELLIPSE.cy + ELLIPSE.ry

  // 边界情况：完全空或完全满
  if (waterY >= ellipseBottom) {
    return {
      waterPath: '',
      emptyPath: `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                 A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                 A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`,
      isEmpty: true,
      isFull: false
    }
  }

  if (waterY <= ellipseTop) {
    return {
      waterPath: `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                  A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                  A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`,
      emptyPath: '',
      isEmpty: false,
      isFull: true
    }
  }

  // 计算水位线与椭圆的交点
  const intersection = calculateEllipseIntersection(waterY, ELLIPSE.cx, ELLIPSE.cy, ELLIPSE.rx, ELLIPSE.ry)

  if (!intersection) {
    // 不应该到达这里，但作为 fallback
    return {
      waterPath: '',
      emptyPath: '',
      isEmpty: true,
      isFull: false
    }
  }

  // 生成有水部分路径（下半椭圆 + 水平线）
  const waterPath = `M ${intersection.left} ${waterY}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 0 1 ${intersection.right} ${waterY}
                     L ${intersection.left} ${waterY}`

  // 生成无水部分路径（上半椭圆 + 水平线）
  const emptyPath = `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 0 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 0 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`

  return {
    waterPath,
    emptyPath,
    isEmpty: false,
    isFull: false
  }
}
```

**Step 2: 提交**

```bash
git add src/utils/jellySpiritWaterPath.ts
git commit -m "feat(utils): add JellySpirit water path generator"
```

---

## Task 3: 实现 PixelGhost 幽灵水位路径生成

**文件:**
- Create: `src/utils/pixelGhostWaterPath.ts`

**Step 1: 创建 PixelGhost 水位路径生成器**

创建 `src/utils/pixelGhostWaterPath.ts`：

```typescript
import { calculateWaterY, type WaterPathResult } from './waterLevel'

/**
 * 生成 PixelGhost 的水位路径
 * @param usagePercent 用量百分比 (0-100)
 */
export function generatePixelGhostWaterPath(usagePercent: number): WaterPathResult {
  // PixelGhost viewBox: 0 0 64 64
  const VIEW_BOX = { width: 64, height: 64, bottom: 64 }

  // 计算水位线 Y 坐标
  const waterY = calculateWaterY(usagePercent, VIEW_BOX.height, VIEW_BOX.bottom)

  // PixelGhost 身体路径的关键点
  const BODY = {
    topY: 14,           // 顶部 Y
    bottomY: 46,        // 底部波浪的最高点
    leftX: 14,          // 左侧 X
    rightX: 50,         // 右侧 X
    centerX: 32         // 中心 X
  }

  // 边界情况
  if (waterY >= BODY.bottomY) {
    // 完全空
    return {
      waterPath: '',
      emptyPath: getOriginalGhostPath(),
      isEmpty: true,
      isFull: false
    }
  }

  if (waterY <= BODY.topY) {
    // 完全满
    return {
      waterPath: getOriginalGhostPath(),
      emptyPath: '',
      isEmpty: false,
      isFull: true
    }
  }

  // 正常情况：生成水位切割路径
  const { waterPath, emptyPath } = generateSplitGhostPaths(waterY)

  return {
    waterPath,
    emptyPath,
    isEmpty: false,
    isFull: false
  }
}

/**
 * 获取原始 PixelGhost 完整路径
 */
function getOriginalGhostPath(): string {
  return `M 18 14
          Q 32 12 46 14
          Q 50 14 50 18
          L 50 42
          Q 50 46 46 46
          Q 44 48 42 46
          Q 40 44 38 46
          Q 36 48 34 46
          Q 32 44 30 46
          Q 28 48 26 46
          Q 24 44 22 46
          Q 20 48 18 46
          Q 14 46 14 42
          L 14 18
          Q 14 14 18 14
          Z`
}

/**
 * 根据水位线生成分割路径
 */
function generateSplitGhostPaths(waterY: number): { waterPath: string; emptyPath: string } {
  // 简化实现：在指定 Y 坐标处水平切割
  // 左侧切割点 (18, waterY)
  // 右侧切割点 (50, waterY)

  const leftX = 18
  const rightX = 50

  // 有水部分：从底部向上到水位线
  const waterPath = `M ${leftX} ${waterY}
                     L ${leftX} 42
                     Q 14 42 14 42
                     L 14 18
                     Q 14 14 18 14
                     Q 32 12 46 14
                     Q 50 14 50 18
                     L 50 42
                     Q 50 46 46 46
                     Q 44 48 42 46
                     Q 40 44 38 46
                     Q 36 48 34 46
                     Q 32 44 30 46
                     Q 28 48 26 46
                     Q 24 44 22 46
                     Q 20 48 18 46
                     Q 14 46 14 42
                     L ${leftX} 42
                     L ${leftX} ${waterY}
                     L ${rightX} ${waterY}
                     Z`

  // 无水部分：从水位线向上到顶部
  const emptyPath = `M 18 14
                     Q 32 12 46 14
                     Q 50 14 50 18
                     L 50 ${waterY}
                     L 18 ${waterY}
                     Q 14 14 18 14
                     Z`

  return { waterPath, emptyPath }
}
```

**Step 2: 提交**

```bash
git add src/utils/pixelGhostWaterPath.ts
git commit -m "feat(utils): add PixelGhost water path generator"
```

---

## Task 4: 修改 JellySpirit 组件支持水位效果

**文件:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 添加水位相关 Props**

```typescript
import { computed } from 'vue'
import { generateJellySpiritWaterPath } from '@/utils/jellySpiritWaterPath'

interface Props {
  color: string
  strokeColor: string
  eyeColor?: string
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'
  waterLevel?: number      // 新增：水位百分比 (0-100)
  emptyColor?: string      // 新增：空罐颜色
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100,
  waterLevel: 100,         // 默认满水
  emptyColor: 'rgba(229, 231, 235, 0.5)'  // 半透明浅灰
})
```

**Step 2: 添加水位路径计算**

```typescript
// 计算水位路径
const waterPaths = computed(() => {
  const usagePercent = 100 - props.waterLevel  // 转换：水位 → 用量
  return generateJellySpiritWaterPath(usagePercent)
})
```

**Step 3: 修改模板渲染两层路径**

找到 `<ellipse>` 身体元素（第63行），替换为：

```vue
<!-- 无水部分（空罐） -->
<path
  v-if="!waterPaths.isFull"
  :fill="props.emptyColor"
  :d="waterPaths.emptyPath"
/>

<!-- 有水部分 -->
<ellipse
  v-if="waterPaths.isFull"
  cx="32"
  cy="36"
  rx="24"
  ry="26"
  :fill="props.color"
  :stroke="props.strokeColor"
  stroke-width="2"
>
  <!-- 保留原有动画 -->
</ellipse>

<path
  v-if="!waterPaths.isFull && !waterPaths.isEmpty"
  :fill="props.color"
  :stroke="props.strokeColor"
  stroke-width="2"
  :d="waterPaths.waterPath"
>
  <!-- 保留原有动画 -->
  <animateTransform
    v-if="state === 'Fresh'"
    attributeName="transform"
    type="scale"
    values="1,1; 1,1.05; 1,1"
    dur="3s"
    repeatCount="indefinite"
    calcMode="spline"
    keyTimes="0;0.5;1"
    keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
  />
</path>
```

**Step 4: 添加水位过渡动画**

```vue
<style scoped>
.spirit-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 9px;
}

.spirit-svg {
  display: block;
}

/* 新增：水位路径平滑过渡 */
.water-path {
  transition: d 0.3s ease-out;
}
</style>
```

给路径元素添加 class：

```vue
<path
  v-if="!waterPaths.isFull && !waterPaths.isEmpty"
  class="water-path"
  :fill="props.color"
  :stroke="props.strokeColor"
  stroke-width="2"
  :d="waterPaths.waterPath"
>
  <!-- ... -->
</path>
```

**Step 5: 提交**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat(pet): JellySpirit add water level effect with dynamic path"
```

---

## Task 5: 修改 PixelGhost 组件支持水位效果

**文件:**
- Modify: `src/components/pets/PixelGhost.vue`

**Step 1: 添加水位相关 Props**

```typescript
import { computed } from 'vue'
import { generatePixelGhostWaterPath } from '@/utils/pixelGhostWaterPath'

interface Props {
  color: string
  strokeColor: string
  eyeColor?: string
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'
  waterLevel?: number      // 新增
  emptyColor?: string      // 新增
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100,
  waterLevel: 100,
  emptyColor: 'rgba(229, 231, 235, 0.5)'
})
```

**Step 2: 添加水位路径计算**

```typescript
const waterPaths = computed(() => {
  const usagePercent = 100 - props.waterLevel
  return generatePixelGhostWaterPath(usagePercent)
})
```

**Step 3: 修改模板渲染两层路径**

找到 `<path>` 身体元素（第74行），替换为：

```vue
<!-- 无水部分（空罐） -->
<path
  v-if="!waterPaths.isFull"
  :fill="props.emptyColor"
  :stroke="props.strokeColor"
  stroke-width="2"
  stroke-linejoin="round"
  stroke-linecap="round"
  :d="waterPaths.emptyPath"
/>

<!-- 有水部分 -->
<path
  v-if="waterPaths.isFull"
  :fill="props.color"
  :stroke="props.strokeColor"
  stroke-width="2"
  stroke-linejoin="round"
  stroke-linecap="round"
  d="M 18 14 Q 32 12 46 14 Q 50 14 50 18 L 50 42 Q 50 46 46 46 Q 44 48 42 46 Q 40 44 38 46 Q 36 48 34 46 Q 32 44 30 46 Q 28 48 26 46 Q 24 44 22 46 Q 20 48 18 46 Q 14 46 14 42 L 14 18 Q 14 14 18 14 Z"
>
  <!-- 保留原有动画 -->
</path>

<path
  v-if="!waterPaths.isFull && !waterPaths.isEmpty"
  class="water-path"
  :fill="props.color"
  :stroke="props.strokeColor"
  stroke-width="2"
  stroke-linejoin="round"
  stroke-linecap="round"
  :d="waterPaths.waterPath"
>
  <!-- 保留原有动画 -->
  <animateTransform
    v-if="state === 'Fresh'"
    attributeName="transform"
    type="scale"
    values="1,1; 1.02,1.02; 1,1"
    dur="2.5s"
    repeatCount="indefinite"
    calcMode="spline"
    keyTimes="0;0.5;1"
    keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
  />
  <animateTransform
    v-if="state === 'Panic'"
    attributeName="transform"
    type="scale"
    values="1,1; 1.05,1.05; 1,1"
    dur="0.3s"
    repeatCount="indefinite"
    calcMode="spline"
    keyTimes="0;0.5;1"
    keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
  />
</path>
```

**Step 4: 添加过渡动画样式**

```vue
<style scoped>
.ghost-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 5px;
}

.ghost-svg {
  display: block;
}

.water-path {
  transition: d 0.3s ease-out;
}
</style>
```

**Step 5: 提交**

```bash
git add src/components/pets/PixelGhost.vue
git commit -m "feat(pet): PixelGhost add water level effect with dynamic path"
```

---

## Task 6: 更新 PetWidget 传递水位数据

**文件:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 计算水位百分比**

找到 `useUsageState` 的调用位置，添加水位计算：

```typescript
const { usagePercent, petState, gradientColor, gradientStrokeColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 新增：水位 = 100 - 用量百分比
const waterLevel = computed(() => 100 - usagePercent.value)
```

**Step 2: 传递水位 prop 给宠物组件**

更新宠物组件的 props 绑定：

```vue
<JellySpirit
  v-if="petType === 'spirit'"
  :color="gradientColor"
  :stroke-color="gradientStrokeColor"
  :state="petState"
  :water-level="waterLevel"
  :width="100"
  :height="100"
/>

<PixelGhost
  v-else-if="petType === 'ghost'"
  :color="gradientColor"
  :stroke-color="gradientStrokeColor"
  :state="petState"
  :water-level="waterLevel"
  :width="100"
  :height="100"
/>
```

**Step 3: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "feat(pet): pass water level to pet components"
```

---

## Task 7: 手动测试验证

**Step 1: 启动开发服务器**

运行: `npm run tauri:dev`
预期: 应用正常启动，显示宠物

**Step 2: 测试不同用量的水位效果**

修改 `src-tauri/src/polling.rs` 中的 mock 数据：

```rust
// 测试 0% 用量 = 100% 水位（满）
UsageData { used: 0, total: 100, ..default() }

// 测试 25% 用量 = 75% 水位
UsageData { used: 25, total: 100, ..default() }

// 测试 50% 用量 = 50% 水位
UsageData { used: 50, total: 100, ..default() }

// 测试 75% 用量 = 25% 水位
UsageData { used: 75, total: 100, ..default() }

// 测试 100% 用量 = 0% 水位（空）
UsageData { used: 100, total: 100, ..default() }
```

**Step 3: 验证视觉效果**

- [ ] 0% 用量：宠物完全填充颜色（满水）
- [ ] 50% 用量：宠物上下各半，下半部分有颜色，上半部分半透明
- [ ] 100% 用量：宠物几乎完全半透明（空水）
- [ ] 水位变化时有平滑过渡动画
- [ ] 原有的状态动画（呼吸、摇摆等）正常工作

**Step 4: 记录测试结果**

创建 `test-notes-water-level.md`：

```markdown
# 储水罐效果测试记录

## 测试日期
2026-04-11

## 测试结果
- [ ] 0% 用量 → 100% 水位（满）
- [ ] 25% 用量 → 75% 水位
- [ ] 50% 用量 → 50% 水位
- [ ] 75% 用量 → 25% 水位
- [ ] 100% 用量 → 0% 水位（空）
- [ ] 水位过渡动画平滑
- [ ] 原有动画正常

## 问题记录
（记录发现的问题）
```

**Step 5: 提交测试记录**

```bash
git add test-notes-water-level.md
git commit -m "test: add water level effect test notes"
```

---

## Task 8: 代码审查和清理

**Step 1: 检查类型定义**

确认 `src/types/config.ts` 无需更新。

**Step 2: 运行类型检查**

运行: `npx vue-tsc --noEmit`
预期: 无类型错误

**Step 3: 检查未使用的代码**

运行: `grep -r "currentColors" src/`
预期: 无残留引用（已在之前的任务中清理）

**Step 4: 构建测试**

运行: `npm run tauri:build`
预期: 构建成功

**Step 5: 最终提交**

```bash
git add .
git commit -m "test: verify water level effect implementation"
```

---

## 验收标准

- [ ] 两个宠物组件都支持水位效果
- [ ] 0% 用量显示满水（100% 填充）
- [ ] 100% 用量显示空水（0% 填充）
- [ ] 水位变化时有平滑过渡动画
- [ ] 所有原有动画效果正常工作
- [ ] 类型检查通过
- [ ] 构建成功

## 参考文档

- 设计文档: `docs/plans/2026-04-11-water-tank-effect-design.md`
- SVG Path 命令: https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
- 椭圆方程: https://en.wikipedia.org/wiki/Ellipse
