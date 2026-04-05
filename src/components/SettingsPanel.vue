<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettings } from '../composables/useSettings'

const {
  config,
  isLoading,
  error,
  testResult,
  activeModel,
  availableModels,
  loadConfig,
  saveConfig,
  switchModel,
  updateModelConfig,
  testConnection,
  setupConfigListener
} = useSettings()

// UI 状态
const activeTab = ref<'models' | 'polling'>('models')
const isTesting = ref(false)
const showApiKey = ref<Record<string, boolean>>({})

// 关闭窗口
async function closeWindow() {
  try {
    await invoke('close_settings_panel')
  } catch (err) {
    console.error('Close window failed:', err)
  }
}

// 保存并关闭
async function saveAndClose() {
  try {
    await saveConfig()
    await closeWindow()
  } catch (err) {
    console.error('Save failed:', err)
  }
}

// 测试连接
async function handleTestConnection(model: any) {
  try {
    isTesting.value = true
    // 硬编码的 API 域名
    const domains: Record<string, string> = {
      'bigmodel': 'https://open.bigmodel.cn/',
      'zai': 'https://api.z.ai/'
    }
    const domain = domains[model.provider] || 'https://open.bigmodel.cn/'
    const apiUrl = domain + 'api/monitor/usage/quota/limit'
    await testConnection(apiUrl, model.api_key)
  } catch (err) {
    console.error('Test connection failed:', err)
  } finally {
    isTesting.value = false
  }
}

// 切换 API Key 显示
function toggleApiKeyVisibility(provider: string) {
  showApiKey.value[provider] = !showApiKey.value[provider]
}

// 组件挂载
let cleanup: (() => void) | undefined

onMounted(async () => {
  await loadConfig()
  cleanup = await setupConfigListener()
})

onUnmounted(() => {
  cleanup?.()
})

onUnmounted(() => {
  cleanup?.()
})
</script>

<template>
  <div class="settings-panel rpg-panel">
    <!-- RPG 风格边框装饰 -->
    <div class="rpg-border-tl"></div>
    <div class="rpg-border-tr"></div>
    <div class="rpg-border-bl"></div>
    <div class="rpg-border-br"></div>

    <!-- 标题栏 -->
    <div class="rpg-header">
      <span class="rpg-title">设置</span>
      <button class="rpg-close" @click="closeWindow">[X]</button>
    </div>

    <!-- 标签切换 -->
    <div class="rpg-tabs">
      <button
        class="rpg-tab"
        :class="{ active: activeTab === 'models' }"
        @click="activeTab = 'models'"
      >
        [模型配置]
      </button>
      <button
        class="rpg-tab"
        :class="{ active: activeTab === 'polling' }"
        @click="activeTab = 'polling'"
      >
        [轮询设置]
      </button>
    </div>

    <div class="rpg-content">
      <!-- 模型配置标签 -->
      <div v-if="activeTab === 'models'" class="tab-content">
        <!-- 当前模型选择 -->
        <div class="rpg-card">
          <div class="rpg-label">当前模型</div>
          <select
            class="rpg-select"
            :value="config.api_config.current_model"
            @change="switchModel(($event.target as HTMLSelectElement).value)"
          >
            <option
              v-for="model in config.api_config.models.filter(m => m.enabled)"
              :key="model.provider"
              :value="model.provider"
            >
              {{ model.name }}
            </option>
          </select>
        </div>

        <!-- 模型列表 -->
        <div class="rpg-card">
          <div class="rpg-label">模型列表</div>
          <div class="rpg-models-list">
            <div
              v-for="model in config.api_config.models"
              :key="model.provider"
              class="rpg-model-item"
              :class="{ disabled: !model.enabled }"
            >
              <div class="rpg-model-info">
                <span class="rpg-model-name">{{ model.name }}</span>
                <label class="rpg-switch">
                  <input
                    type="checkbox"
                    :checked="model.enabled"
                    @change="updateModelConfig(model.provider, { enabled: ($event.target as HTMLInputElement).checked })"
                  />
                  <span class="rpg-switch-slider"></span>
                </label>
              </div>
              <div v-if="model.enabled" class="rpg-model-apikey">
                <label class="rpg-field-label">API 密钥</label>
                <div class="rpg-input-group">
                  <input
                    :type="showApiKey[model.provider] ? 'text' : 'password'"
                    class="rpg-input"
                    :value="model.api_key"
                    @input="updateModelConfig(model.provider, { api_key: ($event.target as HTMLInputElement).value })"
                  />
                  <button
                    class="rpg-icon-btn"
                    @click="toggleApiKeyVisibility(model.provider)"
                  >
                    {{ showApiKey[model.provider] ? '[👁️]' : '[🔒]' }}
                  </button>
                  <button
                    class="rpg-button-small"
                    :disabled="isTesting"
                    @click="handleTestConnection(model)"
                  >
                    {{ isTesting ? '[...]' : '[测试]' }}
                  </button>
                </div>
                <div v-if="testResult && activeModel?.provider === model.provider" class="rpg-test-result">
                  <span :class="{ success: testResult.success, error: !testResult.success }">
                    {{ testResult.success ? '[成功]' : '[失败]' }}
                  </span>
                  {{ testResult.message }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 轮询配置标签 -->
      <div v-if="activeTab === 'polling'" class="tab-content">
        <div class="rpg-card">
          <div class="rpg-label">轮询间隔（分钟）</div>
          <input
            type="number"
            class="rpg-input"
            :value="config.polling_config.interval_minutes"
            @input="config.polling_config.interval_minutes = parseInt(($event.target as HTMLInputElement).value) || 1"
            min="1"
            max="60"
          />
          <div class="rpg-help">
            当前：{{ config.polling_config.interval_minutes }} 分钟
          </div>
        </div>

        <div class="rpg-card">
          <div class="rpg-label">说明</div>
          <div class="rpg-info-text">
            更改将在下次轮询时生效。
          </div>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="rpg-error">
        ! 错误：{{ error }}
      </div>

      <!-- 底部操作按钮 -->
      <div class="rpg-footer-actions">
        <button class="rpg-button-secondary" @click="closeWindow">
          [取消]
        </button>
        <button class="rpg-button-primary" @click="saveAndClose" :disabled="isLoading">
          {{ isLoading ? '[...]' : '[保存]' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

/* RPG 面板基础样式 */
.settings-panel.rpg-panel {
  width: 100vw;
  height: 100vh;
  background: #1a1a2e;
  background-image:
    repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(0, 0, 0, 0.1) 2px, rgba(0, 0, 0, 0.1) 4px),
    repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(0, 0, 0, 0.1) 2px, rgba(0, 0, 0, 0.1) 4px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: 'Press Start 2P', monospace;
  position: relative;
  color: #e0e0e0;
}

/* 边框装饰 */
.rpg-border-tl,
.rpg-border-tr,
.rpg-border-bl,
.rpg-border-br {
  position: absolute;
  width: 16px;
  height: 16px;
  border: 2px solid #ffd700;
  pointer-events: none;
  z-index: 10;
}

.rpg-border-tl { top: 6px; left: 6px; border-right: none; border-bottom: none; }
.rpg-border-tr { top: 6px; right: 6px; border-left: none; border-bottom: none; }
.rpg-border-bl { bottom: 6px; left: 6px; border-right: none; border-top: none; }
.rpg-border-br { bottom: 6px; right: 6px; border-left: none; border-top: none; }

/* 标题栏 */
.rpg-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px 8px;
  background: linear-gradient(180deg, #16213e 0%, #1a1a2e 100%);
  border-bottom: 2px solid #ffd700;
  margin: 0 6px;
}

.rpg-title {
  font-size: 11px;
  color: #ffd700;
  text-shadow: 1px 1px 0 #000;
  letter-spacing: 1px;
}

.rpg-close {
  font-family: 'Press Start 2P', monospace;
  font-size: 10px;
  color: #ff6b6b;
  background: transparent;
  border: 2px solid #ff6b6b;
  padding: 3px 6px;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-close:hover {
  background: #ff6b6b;
  color: #fff;
}

/* 标签切换 */
.rpg-tabs {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.2);
}

.rpg-tab {
  font-family: 'Press Start 2P', monospace;
  font-size: 8px;
  color: #948a77;
  background: transparent;
  border: 2px solid #4a4e69;
  padding: 6px 10px;
  cursor: pointer;
  transition: all 0.1s;
  flex: 1;
}

.rpg-tab.active {
  color: #ffd700;
  border-color: #ffd700;
  background: rgba(255, 215, 0, 0.1);
}

.rpg-tab:hover:not(.active) {
  border-color: #7dd3fc;
  color: #7dd3fc;
}

/* 内容区 */
.rpg-content {
  flex: 1;
  padding: 12px 16px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.tab-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* 卡片样式 */
.rpg-card {
  background: #0f0f1a;
  border: 2px solid #4a4e69;
  padding: 10px 12px;
  position: relative;
}

.rpg-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, #ffd700, #ffed4a, #ffd700);
}

.rpg-label {
  font-size: 8px;
  color: #948a77;
  margin-bottom: 8px;
  letter-spacing: 1px;
}

/* 模型选择器 */
.rpg-select {
  width: 100%;
  font-family: 'Press Start 2P', monospace;
  font-size: 8px;
  color: #e0e0e0;
  background: #000;
  border: 2px solid #4a4e69;
  padding: 6px 8px;
  outline: none;
  transition: border-color 0.1s;
  cursor: pointer;
}

.rpg-select:focus {
  border-color: #ffd700;
}

.rpg-select option {
  background: #16213e;
  color: #e0e0e0;
}

/* 模型列表 */
.rpg-models-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.rpg-model-item {
  background: #16213e;
  border: 2px solid #4a4e69;
  padding: 8px 10px;
  transition: all 0.1s;
}

.rpg-model-item.disabled {
  opacity: 0.5;
  border-color: #2a2e3a;
}

.rpg-model-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.rpg-model-name {
  font-size: 9px;
  color: #e0e0e0;
  font-weight: bold;
}

.rpg-switch {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  user-select: none;
}

.rpg-switch input[type="checkbox"] {
  display: none;
}

.rpg-switch-slider {
  width: 32px;
  height: 16px;
  background: #4a4e69;
  border-radius: 2px;
  position: relative;
  transition: background 0.2s;
}

.rpg-switch-slider::before {
  content: '';
  position: absolute;
  width: 12px;
  height: 12px;
  background: #948a77;
  border-radius: 2px;
  top: 2px;
  left: 2px;
  transition: transform 0.2s;
}

.rpg-switch input[type="checkbox"]:checked + .rpg-switch-slider {
  background: #10b981;
}

.rpg-switch input[type="checkbox"]:checked + .rpg-switch-slider::before {
  transform: translateX(16px);
}

.rpg-model-apikey {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px dashed #4a4e69;
}

/* 字段输入 */
.model-config {
  margin-top: 8px;
}

.rpg-field {
  margin-bottom: 10px;
}

.rpg-field-label {
  display: block;
  font-size: 7px;
  color: #7dd3fc;
  margin-bottom: 4px;
}

.rpg-input {
  width: 100%;
  font-family: 'Press Start 2P', monospace;
  font-size: 7px;
  color: #e0e0e0;
  background: #000;
  border: 2px solid #4a4e69;
  padding: 6px 8px;
  outline: none;
  transition: border-color 0.1s;
}

.rpg-input:focus {
  border-color: #ffd700;
}

.rpg-input-group {
  display: flex;
  gap: 4px;
}

.rpg-input-group .rpg-input {
  flex: 1;
}

.rpg-icon-btn {
  font-family: 'Press Start 2P', monospace;
  font-size: 8px;
  color: #948a77;
  background: #16213e;
  border: 2px solid #4a4e69;
  padding: 6px 8px;
  cursor: pointer;
  transition: all 0.1s;
  white-space: nowrap;
}

.rpg-icon-btn:hover {
  border-color: #ffd700;
  color: #ffd700;
}

/* 操作按钮 */
.rpg-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.rpg-button-small {
  font-family: 'Press Start 2P', monospace;
  font-size: 7px;
  color: #ffd700;
  background: #16213e;
  border: 2px solid #ffd700;
  padding: 4px 8px;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-button-small:hover:not(:disabled) {
  background: #ffd700;
  color: #000;
}

.rpg-button-small:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.rpg-test-result {
  font-size: 7px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.rpg-test-result .success {
  color: #10b981;
}

.rpg-test-result .error {
  color: #ff6b6b;
}

/* 间隔选择器 */
.rpg-interval-selector {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.rpg-interval-btn {
  font-family: 'Press Start 2P', monospace;
  font-size: 7px;
  color: #e0e0e0;
  background: #16213e;
  border: 2px solid #4a4e69;
  padding: 4px 8px;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-interval-btn.active {
  color: #ffd700;
  border-color: #ffd700;
  background: rgba(255, 215, 0, 0.1);
}

.rpg-interval-btn:hover:not(.active) {
  border-color: #7dd3fc;
  color: #7dd3fc;
}

.rpg-help {
  font-size: 7px;
  color: #948a77;
  margin-top: 8px;
}

.rpg-info-text {
  font-size: 8px;
  color: #7dd3fc;
  line-height: 1.4;
}

/* 错误提示 */
.rpg-error {
  font-size: 8px;
  color: #ff6b6b;
  background: rgba(255, 107, 107, 0.1);
  border: 2px solid #ff6b6b;
  padding: 6px;
  text-align: center;
}

/* 底部操作按钮 */
.rpg-footer-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding-top: 8px;
  border-top: 1px solid #4a4e69;
}

.rpg-button-primary,
.rpg-button-secondary {
  font-family: 'Press Start 2P', monospace;
  font-size: 9px;
  color: #ffd700;
  background: #16213e;
  border: 2px solid #ffd700;
  padding: 8px 12px;
  cursor: pointer;
  transition: all 0.1s;
  text-shadow: 1px 1px 0 #000;
}

.rpg-button-secondary {
  color: #948a77;
  border-color: #4a4e69;
}

.rpg-button-primary:hover:not(:disabled) {
  background: #ffd700;
  color: #000;
}

.rpg-button-secondary:hover {
  border-color: #7dd3fc;
  color: #7dd3fc;
}

.rpg-button-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 滚动条 */
.rpg-content::-webkit-scrollbar {
  width: 6px;
}

.rpg-content::-webkit-scrollbar-track {
  background: #0f0f1a;
}

.rpg-content::-webkit-scrollbar-thumb {
  background: #4a4e69;
  border: 1px solid #ffd700;
}

.rpg-content::-webkit-scrollbar-thumb:hover {
  background: #ffd700;
}
</style>
