# 储水罐效果实现记录

**日期**: 2026-04-11
**状态**: 代码实现完成，等待用户测试验证

## 实现摘要

成功为 PixelGhost 和 JellySpirit 宠物组件添加了"储水罐"效果，宠物作为储水罐，用量百分比代表"剩余水量"：
- **0% 用量** → 水位满（100% 填充，储水罐满）
- **100% 用量** → 水位空（0% 填充，储水罐用完）

## 实现的任务

### ✅ Task 1: 添加水位计算工具函数
**文件**: `src/utils/waterLevel.ts`

实现了三个核心函数：
- `calculateFillRatio()` - 计算填充比例
- `calculateWaterY()` - 计算水位线 Y 坐标
- `calculateEllipseIntersection()` - 计算椭圆与水平线交点
- `WaterPathResult` 接口定义

### ✅ Task 2: 实现 JellySpirit 椭圆水位路径生成
**文件**: `src/utils/jellySpiritWaterPath.ts`

- `generateJellySpiritWaterPath()` - 为椭圆形的 JellySpirit 生成水位路径
- 处理三种边界情况：完全空、完全满、部分填充
- 使用数学公式计算椭圆与水位线的交点

### ✅ Task 3: 实现 PixelGhost 幽灵水位路径生成
**文件**: `src/utils/pixelGhostWaterPath.ts`

- `generatePixelGhostWaterPath()` - 为波浪形底部的 PixelGhost 生成水位路径
- `getOriginalGhostPath()` - 返回完整的幽灵路径
- `generateSplitGhostPaths()` - 根据水位线生成分割路径

### ✅ Task 4: 修改 JellySpirit 组件支持水位效果
**文件**: `src/components/pets/JellySpirit.vue`

- 添加 Props: `waterLevel` 和 `emptyColor`
- 添加 `waterPaths` 计算属性
- 将单一 `<ellipse>` 替换为三层结构（空罐、满水、部分填充）
- 保留所有原有动画（呼吸、摇摆、抖动等）
- 添加水位过渡动画样式

### ✅ Task 5: 修改 PixelGhost 组件支持水位效果
**文件**: `src/components/pets/PixelGhost.vue`

- 添加 Props: `waterLevel` 和 `emptyColor`
- 添加 `waterPaths` 计算属性
- 将单一 `<path>` 替换为三层结构（空罐、满水、部分填充）
- 保留所有原有动画（漂浮、眨眼、脉动等）
- 添加水位过渡动画样式

### ✅ Task 6: 更新 PetWidget 传递水位数据
**文件**: `src/components/PetWidget.vue`

- 添加 `waterLevel` 计算属性（100 - usagePercent）
- 为 JellySpirit 和 PixelGhost 组件传递 `:water-level` prop
- 建立完整的数据流：Rust → usageData → useUsageState → waterLevel → 宠物组件

## 数据流

```
Rust 端轮询 (5分钟)
    ↓
UsageData { used, total }
    ↓
useUsageState.usagePercent
    ↓
waterLevel = 100 - usagePercent
    ↓
PetWidget 传递 :water-level prop
    ↓
宠物组件调用路径生成工具
    ↓
渲染两层 SVG 路径（无水 + 有水）
    ↓
显示储水罐效果
```

## 视觉效果

### 水位与用量关系
| 用量百分比 | 水位百分比 | 视觉效果 |
|-----------|-----------|---------|
| 0% | 100% | 完全满，宠物全身显示原色 |
| 25% | 75% | 3/4 满水 |
| 50% | 50% | 一半水，一半空罐 |
| 75% | 25% | 1/4 满水 |
| 100% | 0% | 完全空，显示半透明浅灰色 |

### 颜色说明
- **有水部分**：显示宠物原色（随用量渐变）
- **无水部分**：显示 `rgba(229, 231, 235, 0.5)` 半透明浅灰色

### 动画效果
- 水位变化时有 0.3s 平滑过渡动画
- 所有原有的状态动画（呼吸、摇摆、眨眼等）完全保留

## 待测试项目

用户可以通过以下方式测试效果：

### 方法 1: 修改 Mock 数据
编辑 `src-tauri/src/polling.rs` 中的 mock 数据：
```rust
// 测试 0% 用量（满水）
UsageData { used: 0, total: 100, ..default() }

// 测试 50% 用量（半水）
UsageData { used: 50, total: 100, ..default() }

// 测试 100% 用量（空水）
UsageData { used: 100, total: 100, ..default() }
```

### 方法 2: 启动应用
```bash
npm run tauri:dev
```

### 验收清单
- [ ] 0% 用量显示满水（宠物完全着色）
- [ ] 50% 用量显示半水（上下各半）
- [ ] 100% 用量显示空水（半透明浅灰色）
- [ ] 水位变化时有平滑过渡动画
- [ ] 原有动画（呼吸、摇摆等）正常工作
- [ ] JellySpirit 和 PixelGhost 都正常显示

## 实现的文件

### 新增文件
- `src/utils/waterLevel.ts` - 水位计算工具
- `src/utils/jellySpiritWaterPath.ts` - JellySpirit 路径生成器
- `src/utils/pixelGhostWaterPath.ts` - PixelGhost 路径生成器

### 修改文件
- `src/components/pets/JellySpirit.vue` - 添加水位效果
- `src/components/pets/PixelGhost.vue` - 添加水位效果
- `src/components/PetWidget.vue` - 传递水位数据

## 技术要点

1. **动态 Path 重绘**：根据水位百分比实时计算 SVG 路径
2. **椭圆数学计算**：使用椭圆方程求解水位线交点
3. **Vue 响应式**：使用 computed 属性实现响应式路径计算
4. **平滑过渡**：CSS transition 实现水位变化的平滑动画
5. **向后兼容**：所有原有功能（动画、颜色渐变）完全保留

## 下一步

用户需要：
1. 运行 `npm run tauri:dev` 启动应用
2. 测试不同用量下的水位效果
3. 如有问题，提供反馈以便调整
