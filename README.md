# PlanGuard

PlanGuard 是一个跨平台的 API 配额监控桌面应用，通过可爱的宠物角色和直观的状态动画，实时监控您的 API 使用情况。

## 功能特性

- **实时监控**: 自动轮询 API 配额使用情况，每 5 分钟更新一次
- **可视化状态**: 5 种状态显示（良好、使用中、警告、紧张、耗尽）
- **跨平台支持**: 
  - Windows: 桌面宠物小组件，带霓虹光晕效果
  - macOS: 系统托盘集成和气泡面板
- **状态动画**: 根据使用量显示不同的动画效果
- **错误处理**: 完善的错误处理和日志记录

## 技术栈

### 前端
- Vue 3 + TypeScript
- TailwindCSS
- Tauri API

### 后端
- Rust
- Tauri
- Tokio (异步运行时)

## 安装和运行

### 前置要求

- Node.js 18+
- Rust 1.70+
- npm 或 yarn

### 开发模式

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri:dev
```

### 构建生产版本

```bash
# 构建 Web 前端
npm run build

# 构建 Tauri 应用
npm run tauri build
```

## 项目结构

```
plan-guard/
├── src/                          # Vue 前端代码
│   ├── components/               # Vue 组件
│   │   ├── PetWidget.vue        # Windows 桌面宠物组件
│   │   └── PopoverPanel.vue     # macOS 气泡面板组件
│   ├── composables/             # Vue 组合式函数
│   │   ├── usePlatform.ts       # 平台检测
│   │   ├── useTauriEvents.ts   # Tauri 事件监听
│   │   └── useUsageState.ts    # 使用量状态管理
│   ├── App.vue                  # 主应用组件
│   ├── main.ts                  # 应用入口
│   └── style.css                # 全局样式
├── src-tauri/                   # Rust 后端代码
│   ├── src/
│   │   ├── main.rs              # 应用入口
│   │   ├── events.rs            # 事件定义
│   │   ├── polling.rs           # 轮询逻辑
│   │   ├── commands.rs          # Tauri 命令
│   │   └── tray.rs              # 系统托盘支持
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置
└── package.json                 # Node.js 依赖配置
```

## 配置说明

### Tauri 配置 (src-tauri/tauri.conf.json)

- **窗口大小**: 150x150 像素
- **透明窗口**: 支持透明背景
- **始终置顶**: 窗口保持在其他窗口之上
- **无边框**: 隐藏窗口装饰
- **轮询间隔**: 5 分钟（可在 `polling.rs` 中修改）

### 状态阈值

应用根据使用量百分比显示不同状态：

- **Fresh (0-50%)**: 绿色，呼吸动画
- **Flow (50-75%)**: 蓝色，敲打动画
- **Warning (75-90%)**: 黄色，抖动动画
- **Panic (90-99%)**: 橙色，冒汗动画
- **Dead (100%)**: 灰色，灵魂出窍动画

## 开发指南

### 添加新的 API 端点

修改 `src-tauri/src/polling.rs` 中的 `fetch_usage()` 函数：

```rust
pub async fn fetch_usage() -> Result<UsageData, String> {
    // 替换为实际的 API 调用
    let response = reqwest::get("https://your-api.com/usage")
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let data: UsageData = response.json()
        .await
        .map_err(|e| format!("Parse failed: {}", e))?;
    
    Ok(data)
}
```

### 自定义动画样式

修改 `src/components/PetWidget.vue` 中的 CSS 动画：

```css
@keyframes your-animation {
  0% { /* 初始状态 */ }
  100% { /* 结束状态 */ }
}

.your-state {
  animation: your-animation 2s ease-in-out infinite;
}
```

### 添加新平台支持

1. 在 `src/composables/usePlatform.ts` 中添加平台检测
2. 在 `src/App.vue` 中添加平台特定的组件
3. 在 `src-tauri/src/tray.rs` 中添加平台特定的托盘代码

## 测试

### Rust 测试

```bash
cd src-tauri
cargo test
```

### 前端构建测试

```bash
npm run build
```

## 故障排除

### 编译错误

如果遇到编译错误，请确保：

1. Rust 和 Node.js 版本符合要求
2. 所有依赖已正确安装：`npm install`
3. Rust 工具链已更新：`rustup update`

### Tauri 开发服务器无法启动

1. 检查端口 1420 是否被占用
2. 尝试清除缓存：`npm run clean`
3. 重新安装依赖：`rm -rf node_modules && npm install`

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m 'feat: add some feature'`
4. 推送到分支：`git push origin feature/your-feature`
5. 提交 Pull Request

## 许可证

MIT License

## 联系方式

如有问题或建议，欢迎通过 GitHub Issues 联系我们。

---

**PlanGuard** - 让 API 配额监控变得有趣！
