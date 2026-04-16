# GitHub Actions 自动构建配置指南

## 已创建的文件

✅ `.github/workflows/build.yml` - 自动构建配置文件

## 需要的配置

### 1. 生成 Tauri 密钥对（必须）

在本地运行：

```bash
# 安装 Tauri CLI（如果还没装）
npm install -g @tauri-apps/cli

# 生成密钥对
tauri signer generate
```

这会生成两个文件：
- `tauri-private-key.key` - 私钥（保密！）
- `tauri-public-key.key` - 公钥

### 2. 在 GitHub 仓库中配置密钥

1. 打开 GitHub 仓库页面
2. 进入 **Settings** → **Secrets and variables** → **Actions**
3. 添加以下 Secret：

| Secret 名称 | 内容 |
|-------------|------|
| `TAURI_PRIVATE_KEY` | `tauri-private-key.key` 文件的完整内容 |
| `TAURI_KEY_PASSWORD` | 你设置的密码（如果有的话） |
| `GITHUB_TOKEN` | 自动生成，无需手动添加 |

### 3. 提交配置文件到 GitHub

```bash
cd glm-token-monitor
git add .github/workflows/build.yml
git commit -m "Add GitHub Actions build configuration"
git push
```

## 如何触发构建

### 方式 1：推送标签（推荐）

```bash
# 创建版本标签
git tag v1.0.0

# 推送标签到 GitHub
git push origin v1.0.0
```

标签格式：`v1.0.0`, `v1.1.0`, `v2.0.0` 等

### 方式 2：手动触发

1. 打开 GitHub 仓库
2. 进入 **Actions** 标签
3. 选择 "Build Multi-Platform Release"
4. 点击 **Run workflow**

## 下载构建结果

构建完成后（约 10-20 分钟）：

1. 进入 **Actions** 标签
2. 点击最新的构建任务
3. 滚到底部，在 **Artifacts** 部分下载：
   - `glm-token-monitor-windows` - Windows exe 安装包
   - `glm-token-monitor-macos` - macOS dmg 文件
   - `glm-token-monitor-ubuntu` - Linux deb 文件

或者在 **Releases** 页面下载发布的版本。

## 注意事项

⚠️ **重要**：
- 私钥文件 `tauri-private-key.key` 必须保密，不要提交到代码仓库
- 如果密钥泄露，需要重新生成并更新 GitHub Secret
- 构建过程需要 10-20 分钟，请耐心等待

---

## 问题排查

如果构建失败，检查：
1. Actions 日志中的错误信息
2. Tauri 密钥是否正确配置
3. package.json 中的构建脚本是否正确

---

需要我帮你解决配置问题吗？
