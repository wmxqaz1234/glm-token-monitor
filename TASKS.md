# 卡皮巴拉宠物功能任务清单

## 验收标准
- [x] 卡皮巴拉宠物能正确显示在主窗口
- [x] 5种佛系动画（idle、orange、spa、munch、stare）正常工作
- [x] 动作每25秒自动切换
- [x] 宠物状态变化时颜色正确更新
- [x] 可以通过配置切换到卡皮巴拉宠物

## 任务列表

### Task 1: 创建 Capybara.vue 组件 ✅
**验收**: `src/components/pets/Capybara.vue` 文件存在，包含完整的 SVG 卡皮巴拉形象

**已完成**:
- 使用 SVG 绘制卡皮巴拉（圆润身体、小耳朵、四条短腿）
- 5种动画状态：idle（静坐）、orange（头顶橘子）、spa（泡温泉）、munch（嚼草）、stare（盯着镜头）
- 颜色根据 petState 动态变化（color、strokeColor props）
- 尺寸 80x80，响应 props

### Task 2: 更新 PetWidget.vue 导入和注册 ✅
**验收**: Capybara 组件已导入并在模板中正确使用

**已完成**:
- 在 script 部分导入 Capybara 组件 (第14行)
- 在模板中添加 `v-else-if="petType === 'capybara'"` 分支 (第507行)
- 传递 color、strokeColor、state、action、width、height、accessories props

### Task 3: 验证类型定义已完整 ✅
**验收**: `src/types/config.ts` 中 PetType 包含 'capybara'，CapybaraAction 类型已定义

**已完成**:
- PetType = 'spirit' | 'ghost' | 'capybara' (第30行)
- CapybaraAction 包含所有5种动作 (第33-38行)

### Task 4: 验证 usePetAction.ts 已更新 ✅
**验收**: `src/composables/usePetAction.ts` 支持 capybara 类型

**已完成**:
- CAPYBARA_ACTIONS 数组包含所有5种动作 (第9-15行)
- availableActions computed 支持 capybara (第26行)
- setPetType 函数支持 capybara (第45行)

### Task 5: 运行开发服务器验证 ✅
**验收**: 前端构建成功，无 TypeScript 错误

**已完成**:
- 无 TypeScript 错误
- 前端构建成功 `npm run build` (3.10s)

## 已完成

2024-04-24: 卡皮巴拉宠物功能全部完成，前端构建通过
