# 卡皮巴拉宠物功能开发

你是一个前端工程师，负责为 glm-token-monitor 项目添加卡皮巴拉宠物功能。

## 项目背景
这是一个 Tauri + Vue 3 的桌面应用，通过可爱的宠物展示 API 使用量。

## 当前状态
- `src/types/config.ts` 已添加 `capybara` 到 PetType
- `src/composables/usePetAction.ts` 已添加卡皮巴拉动作系统

## 需要完成的任务

### Task 1: 创建 Capybara.vue 组件
在 `src/components/pets/Capybara.vue` 创建卡皮巴拉组件：

**要求**:
1. 使用 SVG 绘制卡皮巴拉形象：
   - 圆润的棕色身体
   - 小小的圆形耳朵
   - 四条短腿
   - 可爱的表情（豆豆眼、小鼻子）

2. 实现 5 种动画状态：
   - `capybara-idle`: 静坐冥想（默认，几乎不动，轻微呼吸）
   - `capybara-orange`: 头顶一个橙色橘子（SVG 元素）
   - `capybara-spa`: 泡温泉（身体周围有蒸汽和水波纹）
   - `capybara-munch`: 嚼草（嘴巴咀嚼动作）
   - `capybara-stare`: 盯着镜头（眼睛跟随效果）

3. Props 接口：
   ```typescript
   interface Props {
     color: string        // 根据状态变化的颜色
     strokeColor: string  // 描边颜色
     state: string        // petState (Fresh/Flow/Warning/Panic/Exhausted/Dead)
     width: number
     height: number
   }
   ```

4. 动画要求：
   - 所有动画都是缓慢、淡定的（佛系风格）
   - 使用 CSS 动画，不使用 JavaScript
   - 动画时长至少 3-5 秒，体现"慢生活"

### Task 2: 更新 PetWidget.vue
修改 `src/components/PetWidget.vue`：

1. 在 script 部分导入 Capybara：
   ```typescript
   import Capybara from './pets/Capybara.vue'
   ```

2. 在模板的宠物容器中添加：
   ```vue
   <Capybara v-else-if="petType === 'capybara'"
             :color="gradientColor"
             :stroke-color="gradientStrokeColor"
             :state="petState"
             :width="80"
             :height="80" />
   ```

### Task 3: 验证和测试
1. 运行 `npm run tauri:dev` 验证：
   - 无 TypeScript 错误
   - 卡皮巴拉正确显示
   - 动画正常播放
   - 颜色随状态变化

## 验收标准
- [ ] Capybara.vue 文件存在且代码完整
- [ ] PetWidget.vue 正确导入和使用 Capybara
- [ ] 开发服务器启动无错误
- [ ] 卡皮巴拉宠物可见且动画正常

## 参考现有组件
可以参考 `src/components/pets/JellySpirit.vue` 和 `PixelGhost.vue` 的实现方式。

## 重要提示
- 卡皮巴拉是"佛系动物"，所有动画都要慢、淡定
- 颜色应该动态反映 API 使用状态
- 保持代码风格与现有组件一致
