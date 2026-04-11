# API 配置提示优化说明

## ✨ 优化内容

### 1. 信息面板优化

当用户未配置 API Key 时，信息面板会显示友好的提示：

**显示内容**:
- 🔑 图标（带跳动动画）
- 标题："需要配置 API Key"
- 描述："请先配置 API Key 才能查看使用量"
- "去设置" 按钮

**样式特点**:
- 蓝色渐变背景
- 滑入动画
- 图标跳动动画
- 悬停效果

### 2. 宠物面板对话气泡

当用户未配置 API Key 时，宠物会通过对话气泡提示用户：

**显示内容**:
- 🔑 钥匙图标（左右摇晃动画）
- 文字："请配置 API Key"
- 箭头：→（指向右侧，暗示点击）
- 可点击：点击后直接打开设置面板

**动画效果**:
- 整体气泡：上下浮动 + 缩放脉冲
- 钥匙图标：左右摇晃
- 箭头：左右摆动
- 悬停时：背景变亮，阴影增强

**显示时机**:
- 应用启动 2 秒后自动显示
- 持续 8 秒后自动隐藏
- 不会与心语气泡同时显示

### 3. 智能点击处理

根据 API 配置状态，点击宠物窗口会有不同的行为：

**已配置 API**:
- 点击 → 打开信息面板

**未配置 API**:
- 点击 → 打开设置面板

---

## 🎯 用户体验流程

### 首次使用流程

1. **启动应用**
   - 2 秒后，宠物显示对话气泡："请配置 API Key →"

2. **用户点击宠物或气泡**
   - 自动打开设置面板

3. **在设置中配置 API Key**
   - 填写 API Key
   - 点击"测试连接"验证
   - 点击"保存"

4. **配置完成**
   - 对话气泡消失
   - 应用开始正常显示数据

---

## 📝 技术实现

### 修改的文件

1. **`src/composables/useSettings.ts`**
   - 添加 `hasApiKey` 计算属性
   - 检测当前激活模型的 API Key 是否为空

2. **`src/components/InfoPanel.vue`**
   - 导入 `hasApiKey`
   - 添加 `openSettings` 方法
   - 添加 API 配置提示 UI
   - 添加相关样式

3. **`src/components/PetWidget.vue`**
   - 导入 `hasApiKey`
   - 添加 `showApiConfigBubble` 状态
   - 添加 `openSettings` 方法
   - 修改 `handleClick` 实现智能跳转
   - 添加 API 配置气泡 UI
   - 添加相关动画样式

### 核心逻辑

```typescript
// 检测 API Key 是否配置
const hasApiKey = computed(() => {
  const key = activeModel.value?.api_key || ''
  return key && key.trim().length > 0
})

// 延迟显示配置提示
setTimeout(() => {
  if (!hasApiKey.value) {
    showApiConfigBubble.value = true
    setTimeout(() => {
      showApiConfigBubble.value = false
    }, 8000)
  }
}, 2000)

// 智能点击处理
if (!hasApiKey.value) {
  await invoke('open_settings_panel')
} else {
  await invoke('open_info_panel')
}
```

---

## 🎨 样式细节

### API 配置气泡样式

```css
.api-config-bubble {
  /* 渐变背景 */
  background: linear-gradient(135deg, #1e3a5f 0%, #1a3454 100%);
  
  /* 蓝色边框 */
  border-color: #3b82f6;
  
  /* 多重动画 */
  animation: 
    api-bounce 1s ease-in-out infinite,    /* 缩放脉冲 */
    float-bubble 2s ease-in-out infinite alternate; /* 上下浮动 */
  
  /* 发光效果 */
  box-shadow: 0 0 12px rgba(59, 130, 246, 0.4);
}
```

### 信息面板提示样式

```css
.api-config-notice {
  /* 半透明蓝色背景 */
  background: rgba(59, 130, 246, 0.1);
  
  /* 滑入动画 */
  animation: slideIn 0.3s ease-out;
  
  /* 图标跳动 */
  .notice-icon {
    animation: bounce 2s ease-in-out infinite;
  }
}
```

---

## 🔍 测试场景

### 场景 1：首次使用
1. 启动应用
2. ✅ 2 秒后显示配置提示气泡
3. ✅ 点击宠物或气泡
4. ✅ 打开设置面板
5. ✅ 信息面板显示配置提示

### 场景 2：已配置 API
1. 启动应用
2. ✅ 不显示配置提示气泡
3. ✅ 点击宠物打开信息面板
4. ✅ 正常显示使用量数据

### 场景 3：配置后重启
1. 配置 API Key
2. 重启应用
3. ✅ 不显示配置提示
4. ✅ 正常显示数据

---

## 💡 扩展建议

### 未来可优化的方向

1. **多语言支持**
   - 根据系统语言显示不同提示文案
   - 英文："Please configure API Key"

2. **配置向导**
   - 首次启动显示完整配置向导
   - 步骤化引导用户完成配置

3. **快捷键支持**
   - 快捷键打开设置（如 Ctrl+,）
   - 快捷键刷新数据（如 F5）

4. **状态持久化**
   - 记住是否已显示过配置提示
   - 避免重复打扰用户

5. **错误恢复**
   - API Key 失效时自动提示
   - 提供重新配置的快捷入口

---

## 📊 效果对比

### 优化前
- ❌ 点击宠物打开信息面板
- ❌ 显示错误信息："API returned error code: 1001"
- ❌ 用户不知道如何配置

### 优化后
- ✅ 宠物主动提示："请配置 API Key →"
- ✅ 点击直接跳转到设置面板
- ✅ 信息面板显示友好的配置引导
- ✅ 智能判断，已配置后正常显示数据

---

## 🎁 用户体验提升

1. **主动性**: 宠物主动提示，而非被动等待用户发现错误
2. **清晰性**: 明确告知需要做什么（配置 API Key）
3. **便捷性**: 一键跳转到设置，无需手动查找
4. **友好性**: 可爱的宠物对话，降低配置门槛
5. **智能性**: 自动判断配置状态，已配置后不干扰

---

## 📚 相关文档

- [API配置指南.md](./API配置指南.md) - 详细的 API 配置教程
- [README.md](../README.md) - 项目说明
- [CLAUDE.md](../CLAUDE.md) - 开发指南
