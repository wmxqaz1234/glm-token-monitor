<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useSettings } from '../composables/useSettings'
import { useTheme } from '../composables/useTheme'
import type { DisplayMode } from '../types/config'

const {
  config,
  isLoading,
  error,
  testResult,
  activeModel,
  loadConfig,
  saveConfig,
  switchModel,
  updateModelConfig,
  testConnection,
  setupConfigListener,
  updatePetType,
  updateAutoStart,
  updatePetAccessories,
  thresholdConfig,
  updateThresholds,
  updateColors,
  growthData,
  claimDailyReward
} = useSettings()

const { currentTheme, setTheme } = useTheme()

// UI 状态
const activeTab = ref<'basic' | 'models' | 'pet' | 'threshold' | 'growth'>('basic')
const isDragging = ref(false)

const displayModes: { value: DisplayMode; label: string }[] = [
  { value: 'none', label: '无' },
  { value: 'holo-bubble', label: '全息气泡' },
  { value: 'cyber-ring', label: '赛博光环' },
  { value: 'aura-field', label: '光环力场' },
  { value: 'energy-core', label: '能量核心' },
  { value: 'status-floater', label: '状态悬浮' }
]

const isTesting = ref(false)
const editingApiKey = ref<string | null>(null)
const apiKeyInput = ref('')

// 拖动功能
async function startDrag(event: MouseEvent) {
  // 只在点击header时才允许拖动
  if ((event.target as HTMLElement).closest('.icon-btn')) return

  event.preventDefault()
  isDragging.value = true

  try {
    const win = getCurrentWindow()
    await win.startDragging()
  } catch (error) {
    console.error('拖动失败:', error)
  } finally {
    setTimeout(() => {
      isDragging.value = false
    }, 200)
  }
}

// 实时预览监听
watch(() => config.value?.display_config?.display_mode, async (newMode, oldMode) => {
  if (newMode && newMode !== oldMode) {
    try {
      await saveConfig()
    } catch(e) {
      console.error('saveConfig failed!', e)
    }
  }
})

watch(() => config.value?.pet_config?.selected_pet, async (newPet, oldPet) => {
  if (newPet && newPet !== oldPet) {
    try {
      await saveConfig()
    } catch(e) {
      console.error('saveConfig failed!', e)
    }
  }
})

// 关闭窗口
async function closeWindow() {
  try {
    await invoke('close_settings_panel')
  } catch (err) {
    console.error('Close settings panel failed:', err)
  }
}

// 测试连接
async function handleTestConnection(model: any) {
  try {
    isTesting.value = true
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

// 编辑 API Key
function startEditApiKey(provider: string) {
  const model = config.value?.api_config.models.find(m => m.provider === provider)
  if (model) {
    editingApiKey.value = provider
    apiKeyInput.value = model.api_key || ''
  }
}

// 保存 API Key
function saveApiKey(provider: string) {
  updateModelConfig(provider, { api_key: apiKeyInput.value })
  editingApiKey.value = null
  apiKeyInput.value = ''
  // 自动保存配置
  saveConfig().catch(console.error)
}

// 取消编辑
function cancelEditApiKey() {
  editingApiKey.value = null
  apiKeyInput.value = ''
}

// 临时阈值状态
const tempThresholds = ref({
  fresh: 0,
  flow: 0,
  warning: 0,
  panic: 0
})

// 临时颜色状态
const tempColors = ref({
  fresh: '',
  flow: '',
  warning: '',
  panic: '',
  exhausted: ''
})

// 保存阈值设置
async function handleSaveThresholds() {
  try {
    await updateThresholds({
      fresh_threshold: tempThresholds.value.fresh || thresholdConfig.value?.fresh_threshold || 25,
      flow_threshold: tempThresholds.value.flow || thresholdConfig.value?.flow_threshold || 50,
      warning_threshold: tempThresholds.value.warning || thresholdConfig.value?.warning_threshold || 75,
      panic_threshold: tempThresholds.value.panic || thresholdConfig.value?.panic_threshold || 90
    })
    // 重置临时状态
    tempThresholds.value = { fresh: 0, flow: 0, warning: 0, panic: 0 }
    error.value = null
  } catch (err) {
    error.value = String(err)
  }
}

// 保存颜色设置
async function handleSaveColors() {
  try {
    await updateColors({
      fresh_color: tempColors.value.fresh || undefined,
      flow_color: tempColors.value.flow || undefined,
      warning_color: tempColors.value.warning || undefined,
      panic_color: tempColors.value.panic || undefined,
      exhausted_color: tempColors.value.exhausted || undefined
    })
    // 重置临时状态
    tempColors.value = { fresh: '', flow: '', warning: '', panic: '', exhausted: '' }
    error.value = null
  } catch (err) {
    error.value = String(err)
  }
}

// 更新配件
async function updateAccessories(key: string, value: boolean | string | null) {
  try {
    await updatePetAccessories(key, value)
    // 自动保存配置
    await saveConfig()
  } catch (err) {
    error.value = String(err)
  }
}

// 成长系统相关
const isClaiming = ref(false)

// 等级称号
const levelTitles = [
  '', '新手', '初级', '常用', '资深', '高级', '专家', '大师', '宗师', '传奇', '永恒'
]

function getLevelTitle(level: number): string {
  return levelTitles[level] || '新手'
}

// 等级阈值
const levelThresholds = [0, 500, 1500, 3000, 5000, 8000, 12000, 20000, 30000, 50000]

function getLevelProgressPercent(): number {
  const level = config.value?.growth_data?.level || 1
  const totalXp = config.value?.growth_data?.total_xp || 0
  if (level >= 10) return 100
  const currentThreshold = levelThresholds[level - 1]
  const nextThreshold = levelThresholds[level]
  return Math.round(((totalXp - currentThreshold) / (nextThreshold - currentThreshold)) * 100)
}

function getLevelProgressText(): string {
  const level = config.value?.growth_data?.level || 1
  const totalXp = config.value?.growth_data?.total_xp || 0
  if (level >= 10) return '已达到最高等级'
  const currentThreshold = levelThresholds[level - 1]
  const nextThreshold = levelThresholds[level]
  const current = totalXp - currentThreshold
  return `${current}/${nextThreshold - currentThreshold}`
}

function getTodayXp(): number {
  const maxPercent = config.value?.growth_data?.today_max_percent || 0
  const claimed = config.value?.growth_data?.today_claimed || false
  if (claimed) return 0

  if (maxPercent >= 95) return 150
  if (maxPercent >= 85) return 80
  if (maxPercent >= 70) return 50
  if (maxPercent >= 50) return 30
  return 10
}

async function handleClaimReward() {
  try {
    isClaiming.value = true
    const maxPercent = config.value?.growth_data?.today_max_percent || 0
    const today = new Date().toISOString().split('T')[0]
    const result = await claimDailyReward(maxPercent, today)

    // 重新加载配置
    await loadConfig()

    // 显示升级提示
    if (result.upgraded) {
      alert(`🎉 恭喜升级！获得 ${result.xp} XP，当前等级 Lv.${config.value?.growth_data?.level}`)
    }
  } catch (err) {
    error.value = String(err)
  } finally {
    isClaiming.value = false
  }
}

function getUnlockedCount(): number {
  return config.value?.growth_data?.unlocked_items?.length || 0
}

function getUnlockedItems(): string[] {
  return config.value?.growth_data?.unlocked_items || []
}

function getUnlockIcon(item: string): string {
  const icons: Record<string, string> = {
    'trail-rainbow': '🌈',
    'emoji-basic': '💬',
    'sound-basic': '🎵',
    'particles-stars': '✨',
    'aura-fire': '🔥',
    'aura-frost': '❄️',
    'effect-lightning': '⚡',
    'mask-temp': '🎭',
    'broadcast-status': '📢',
    'color-presets': '🎨',
    'particles-sakura': '🌸',
    'mode-vortex': '🌀',
    'particles-diamond': '💎',
    'transform-butterfly': '🦋',
    'mode-circus': '🎪',
    'mode-wave': '🌊',
    'firework-levelup': '🎆',
    'sound-full': '🔔',
    'skin-night': '🌙',
    'mode-hologram': '🔮',
    'pulse-energy': '⚡',
    'pet-dye': '🎨',
    'particles-legendary': '🌟',
    'crown-legendary': '👑',
    'mode-party': '🎊',
    'transform-unicorn': '🦄',
    'titles': '🏆',
    'pet-rainbow': '🌈',
    'sound-custom': '🎹',
    'custom-infinite': '♾️',
    'mode-cosmic': '🌌',
    'aura-supreme': '👑',
    'mystery-box': '🎁',
    'effects-all': '✨',
    'hall-of-fame': '🏛️'
  }
  return icons[item] || '📦'
}

function getUnlockName(item: string): string {
  const names: Record<string, string> = {
    'trail-rainbow': '彩虹拖尾',
    'emoji-basic': '表情气泡',
    'sound-basic': '基础音效',
    'particles-stars': '星光粒子',
    'aura-fire': '火焰光环',
    'aura-frost': '冰霜光环',
    'effect-lightning': '闪电效果',
    'mask-temp': '宠物面具',
    'broadcast-status': '状态广播',
    'color-presets': '预设配色',
    'particles-sakura': '樱花飘落',
    'mode-vortex': '漩涡模式',
    'particles-diamond': '钻石粒子',
    'transform-butterfly': '蝴蝶变身',
    'mode-circus': '马戏团模式',
    'mode-wave': '波浪模式',
    'firework-levelup': '烟花升级',
    'sound-full': '完整音效',
    'skin-night': '夜间皮肤',
    'mode-hologram': '全息投影',
    'pulse-energy': '能量脉冲',
    'pet-dye': '宠物染色',
    'particles-legendary': '传奇粒子',
    'crown-legendary': '传奇皇冠',
    'mode-party': '派对模式',
    'transform-unicorn': '独角兽变身',
    'titles': '称号系统',
    'pet-rainbow': '彩虹宠物',
    'sound-custom': '自定义音效',
    'custom-infinite': '无限自定义',
    'mode-cosmic': '宇宙模式',
    'aura-supreme': '至尊光环',
    'mystery-box': '神秘宝箱',
    'effects-all': '全特效',
    'hall-of-fame': '名人堂'
  }
  return names[item] || item
}

function getLevelUnlocks(level: number): string[] {
  const unlocks: Record<number, string[]> = {
    2: ['trail-rainbow', 'emoji-basic', 'sound-basic'],
    3: ['particles-stars', 'aura-fire', 'aura-frost', 'effect-lightning'],
    4: ['mask-temp', 'broadcast-status', 'color-presets', 'particles-sakura'],
    5: ['mode-vortex', 'particles-diamond', 'transform-butterfly', 'mode-circus'],
    6: ['mode-wave', 'firework-levelup', 'sound-full', 'skin-night'],
    7: ['mode-hologram', 'pulse-energy', 'pet-dye', 'particles-legendary'],
    8: ['crown-legendary', 'mode-party', 'transform-unicorn', 'titles'],
    9: ['pet-rainbow', 'sound-custom', 'custom-infinite', 'mode-cosmic'],
    10: ['aura-supreme', 'mystery-box', 'effects-all', 'hall-of-fame']
  }
  return unlocks[level] || []
}

// 组件挂载
let cleanup: (() => void) | undefined

onMounted(async () => {
  await loadConfig()
  await setTheme(config.value.basic_config?.theme || 'dark')
  cleanup = await setupConfigListener()
})

onUnmounted(() => {
  cleanup?.()
})
</script>

<template>
  <div class="settings-panel" :data-theme="currentTheme">
    <!-- 顶部栏 -->
    <header class="panel-header" @mousedown="startDrag">
      <div class="header-left">
        <span class="panel-title">设置</span>
      </div>
      <div class="header-right">
        <button class="icon-btn close" @click="closeWindow">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- 标签导航 -->
    <nav class="tab-nav">
      <button
        v-for="tab in [
          { id: 'basic', label: '基础' },
          { id: 'models', label: '模型' },
          { id: 'pet', label: '宠物' },
          { id: 'growth', label: '成长' },
          { id: 'threshold', label: '阈值' }
        ]"
        :key="tab.id"
        class="tab-btn"
        :class="{ active: activeTab === tab.id }"
        @click="activeTab = tab.id as any"
      >
        {{ tab.label }}
      </button>
    </nav>

    <!-- 内容区 -->
    <main class="panel-content">
      <!-- 基础设置 -->
      <div v-if="activeTab === 'basic'" class="settings-section">
        <!-- 主题切换 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">主题</span>
            <span class="setting-desc">选择界面主题</span>
          </div>
          <div class="theme-selector">
            <button
              class="theme-btn"
              :class="{ active: currentTheme === 'dark' }"
              @click="setTheme('dark')"
            >深色</button>
            <button
              class="theme-btn"
              :class="{ active: currentTheme === 'light' }"
              @click="setTheme('light')"
            >浅色</button>
          </div>
        </div>

        <!-- 开机自启 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">开机自启</span>
            <span class="setting-desc">系统启动时自动运行</span>
          </div>
          <label class="toggle-switch">
            <input
              type="checkbox"
              :checked="config.basic_config?.auto_start ?? false"
              @change="updateAutoStart(($event.target as HTMLInputElement).checked)"
            />
            <span class="toggle-track"></span>
            <span class="toggle-thumb"></span>
          </label>
        </div>
      </div>

      <!-- 模型配置 -->
      <div v-if="activeTab === 'models'" class="settings-section">
        <!-- 当前模型 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">当前模型</span>
          </div>
          <div class="model-selector">
            <select
              class="select-input"
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
        </div>

        <!-- 模型列表 -->
        <div class="models-list">
          <div
            v-for="model in config.api_config.models"
            :key="model.provider"
            class="model-item"
            :class="{ active: model.provider === config.api_config.current_model, expanded: editingApiKey === model.provider }"
          >
            <div class="model-main">
              <div class="model-info">
                <span class="model-name">{{ model.name }}</span>
                <span class="model-provider">{{ model.provider }}</span>
              </div>
              <div class="model-actions">
                <button
                  class="action-btn test"
                  :disabled="isTesting"
                  @click="handleTestConnection(model)"
                >
                  {{ isTesting ? '...' : '测试' }}
                </button>
                <button
                  class="action-btn edit-key"
                  @click="startEditApiKey(model.provider)"
                  v-if="editingApiKey !== model.provider"
                >
                  {{ model.api_key ? '修改密钥' : '设置密钥' }}
                </button>
                <label class="toggle-switch mini">
                  <input
                    type="checkbox"
                    :checked="model.enabled"
                    @change="updateModelConfig(model.provider, { enabled: ($event.target as HTMLInputElement).checked })"
                  />
                  <span class="toggle-track"></span>
                  <span class="toggle-thumb"></span>
                </label>
              </div>
            </div>
            <!-- API Key 编辑区域 -->
            <div v-if="editingApiKey === model.provider" class="api-key-editor">
              <div class="api-key-input-group">
                <input
                  type="password"
                  class="api-key-input"
                  v-model="apiKeyInput"
                  placeholder="请输入 API Key"
                  @keyup.enter="saveApiKey(model.provider)"
                  @keyup.esc="cancelEditApiKey()"
                />
                <button class="action-btn save" @click="saveApiKey(model.provider)">保存</button>
                <button class="action-btn cancel" @click="cancelEditApiKey()">取消</button>
              </div>
              <div class="api-key-hint">
                API Key 将被加密存储在本地
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 宠物设置 -->
      <div v-if="activeTab === 'pet'" class="settings-section">
        <!-- 宠物选择 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">宠物类型</span>
            <span class="setting-desc">选择宠物类型</span>
          </div>
          <div class="pet-selector">
            <label
              v-for="pet in [
                { value: 'spirit', label: '果冻精灵', desc: 'Jelly Spirit' },
                { value: 'ghost', label: '像素幽灵', desc: 'Pixel Ghost' }
              ]"
              :key="pet.value"
              class="pet-option"
              :class="{ selected: config.pet_config?.selected_pet === pet.value }"
            >
              <input
                type="radio"
                name="petType"
                :value="pet.value"
                :checked="config.pet_config?.selected_pet === pet.value"
                @change="updatePetType(pet.value)"
              />
              <div class="pet-option-content">
                <span class="pet-label">{{ pet.label }}</span>
                <span class="pet-desc">{{ pet.desc }}</span>
              </div>
            </label>
          </div>
        </div>

        <!-- 展示模式 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">展示模式</span>
            <span class="setting-desc">Token 展示模式</span>
          </div>
          <div class="display-grid">
            <label
              v-for="mode in displayModes"
              :key="mode.value"
              class="display-option"
              :class="{ selected: config.display_config?.display_mode === mode.value }"
            >
              <input
                type="radio"
                name="displayMode"
                :value="mode.value"
                v-model="config.display_config.display_mode"
              />
              <span>{{ mode.label }}</span>
            </label>
          </div>
        </div>

        <!-- 宠物配件 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">宠物配件</span>
            <span class="setting-desc">装饰你的宠物</span>
          </div>
        </div>

        <!-- 配件选择 -->
        <div class="accessories-grid">
          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.sunglasses }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.sunglasses"
              @change="updateAccessories('sunglasses', ($event.target as HTMLInputElement).checked)"
            />
            <span class="accessory-icon">🕶️</span>
            <span class="accessory-label">墨镜</span>
          </label>

          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.bandage }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.bandage"
              @change="updateAccessories('bandage', ($event.target as HTMLInputElement).checked)"
            />
            <span class="accessory-icon">🩹</span>
            <span class="accessory-label">创口贴</span>
          </label>

          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.bow }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.bow"
              @change="updateAccessories('bow', ($event.target as HTMLInputElement).checked)"
            />
            <span class="accessory-icon">🎀</span>
            <span class="accessory-label">蝴蝶结</span>
          </label>

          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.hat === 'cap' }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.hat === 'cap'"
              @change="updateAccessories('hat', ($event.target as HTMLInputElement).checked ? 'cap' : null)"
            />
            <span class="accessory-icon">🧢</span>
            <span class="accessory-label">棒球帽</span>
          </label>

          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.hat === 'beanie' }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.hat === 'beanie'"
              @change="updateAccessories('hat', ($event.target as HTMLInputElement).checked ? 'beanie' : null)"
            />
            <span class="accessory-icon">🎩</span>
            <span class="accessory-label">毛线帽</span>
          </label>

          <label class="accessory-option" :class="{ selected: config.pet_config?.accessories?.hat === 'straw_hat' }">
            <input
              type="checkbox"
              :checked="config.pet_config?.accessories?.hat === 'straw_hat'"
              @change="updateAccessories('hat', ($event.target as HTMLInputElement).checked ? 'straw_hat' : null)"
            />
            <span class="accessory-icon">👒</span>
            <span class="accessory-label">草帽</span>
          </label>
        </div>
      </div>

      <!-- 成长系统 -->
      <div v-if="activeTab === 'growth'" class="settings-section">
        <!-- 等级展示 -->
        <div class="growth-level-card">
          <div class="level-header">
            <div class="level-info">
              <span class="level-number">Lv.{{ config.growth_data?.level || 1 }}</span>
              <span class="level-title">{{ config.growth_data?.level ? getLevelTitle(config.growth_data.level) : '新手' }}</span>
            </div>
            <div class="level-xp">
              <span class="xp-current">{{ config.growth_data?.current_xp || 0 }} XP</span>
            </div>
          </div>

          <!-- 经验进度条 -->
          <div class="xp-progress-container">
            <div class="xp-progress-bar">
              <div class="xp-progress-fill" :style="{ width: getLevelProgressPercent() + '%' }"></div>
            </div>
            <span class="xp-progress-text">{{ getLevelProgressText() }}</span>
          </div>
        </div>

        <!-- 今日奖励 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">今日奖励</span>
            <span class="setting-desc">根据今日用完率领取经验</span>
          </div>
        </div>

        <div class="daily-reward-card">
          <div class="reward-info">
            <div class="reward-percent">今日最高用完率: {{ config.growth_data?.today_max_percent || 0 }}%</div>
            <div class="reward-xp">可获得 {{ getTodayXp() }} XP</div>
          </div>
          <button
            class="claim-btn"
            :class="{ claimed: config.growth_data?.today_claimed, claiming: isClaiming }"
            :disabled="config.growth_data?.today_claimed || isClaiming"
            @click="handleClaimReward"
          >
            {{ config.growth_data?.today_claimed ? '已领取' : '领取奖励' }}
          </button>
        </div>

        <!-- 连续高用完率 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">连续高用完率</span>
            <span class="setting-desc">连续7天保持 ≥70% 用完率获得额外奖励</span>
          </div>
          <div class="streak-display">
            🔥 {{ config.growth_data?.high_usage_streak || 0 }}/7 天
          </div>
        </div>

        <!-- 已解锁特效 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">已解锁特效</span>
            <span class="setting-desc">{{ getUnlockedCount() }} 个解锁内容</span>
          </div>
        </div>

        <div class="unlocks-grid">
          <div
            v-for="item in getUnlockedItems()"
            :key="item"
            class="unlock-item"
            :title="getUnlockName(item)"
          >
            <span class="unlock-icon">{{ getUnlockIcon(item) }}</span>
            <span class="unlock-name">{{ getUnlockName(item) }}</span>
          </div>
          <div v-if="getUnlockedItems().length === 0" class="unlock-empty">
            继续使用可解锁更多特效
          </div>
        </div>

        <!-- 解锁预览 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">解锁预览</span>
            <span class="setting-desc">查看各等级解锁内容</span>
          </div>
        </div>

        <div class="unlock-preview-list">
          <div v-for="level in 9" :key="level" class="unlock-preview-item" :class="{ current: config.growth_data?.level === level }">
            <div class="preview-level">Lv.{{ level + 1 }}</div>
            <div class="preview-items">
              <span v-for="item in getLevelUnlocks(level + 1)" :key="item" class="preview-item">
                {{ getUnlockIcon(item) }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- 阈值设置 -->
      <div v-if="activeTab === 'threshold'" class="settings-section">
        <!-- 阈值滑块 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">状态阈值</span>
            <span class="setting-desc">调整各状态切换的百分比阈值</span>
          </div>
        </div>

        <!-- Fresh 阈值 -->
        <div class="threshold-item">
          <div class="threshold-header">
            <span class="threshold-label" style="color: #1E3A8A">● Fresh (良好)</span>
            <span class="threshold-value">{{ thresholdConfig?.fresh_threshold ?? 25 }}%</span>
          </div>
          <input
            type="range"
            min="5"
            max="40"
            :value="thresholdConfig?.fresh_threshold ?? 25"
            @input="(e) => tempThresholds.fresh = Number((e.target as HTMLInputElement).value)"
            class="threshold-slider"
            style="--slider-color: #1E3A8A"
          />
        </div>

        <!-- Flow 阈值 -->
        <div class="threshold-item">
          <div class="threshold-header">
            <span class="threshold-label" style="color: #0EA5E9">● Flow (使用中)</span>
            <span class="threshold-value">{{ thresholdConfig?.flow_threshold ?? 50 }}%</span>
          </div>
          <input
            type="range"
            :min="(tempThresholds.fresh || thresholdConfig?.fresh_threshold || 25) + 5"
            max="65"
            :value="thresholdConfig?.flow_threshold ?? 50"
            @input="(e) => tempThresholds.flow = Number((e.target as HTMLInputElement).value)"
            class="threshold-slider"
            style="--slider-color: #0EA5E9"
          />
        </div>

        <!-- Warning 阈值 -->
        <div class="threshold-item">
          <div class="threshold-header">
            <span class="threshold-label" style="color: #F59E0B">● Warning (警告)</span>
            <span class="threshold-value">{{ thresholdConfig?.warning_threshold ?? 75 }}%</span>
          </div>
          <input
            type="range"
            :min="(tempThresholds.flow || thresholdConfig?.flow_threshold || 50) + 5"
            max="85"
            :value="thresholdConfig?.warning_threshold ?? 75"
            @input="(e) => tempThresholds.warning = Number((e.target as HTMLInputElement).value)"
            class="threshold-slider"
            style="--slider-color: #F59E0B"
          />
        </div>

        <!-- Panic 阈值 -->
        <div class="threshold-item">
          <div class="threshold-header">
            <span class="threshold-label" style="color: #F97316">● Panic (紧张)</span>
            <span class="threshold-value">{{ thresholdConfig?.panic_threshold ?? 90 }}%</span>
          </div>
          <input
            type="range"
            :min="(tempThresholds.warning || thresholdConfig?.warning_threshold || 75) + 5"
            max="95"
            :value="thresholdConfig?.panic_threshold ?? 90"
            @input="(e) => tempThresholds.panic = Number((e.target as HTMLInputElement).value)"
            class="threshold-slider"
            style="--slider-color: #F97316"
          />
        </div>

        <!-- 保存阈值按钮 -->
        <div class="setting-item">
          <button class="save-btn" @click="handleSaveThresholds">
            保存阈值设置
          </button>
        </div>

        <!-- 自定义颜色 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">自定义颜色</span>
            <span class="setting-desc">留空则使用默认颜色</span>
          </div>
        </div>

        <div class="color-grid">
          <div class="color-item">
            <span class="color-label">Fresh</span>
            <input
              type="color"
              :value="thresholdConfig?.fresh_color || '#1E3A8A'"
              @input="(e) => tempColors.fresh = (e.target as HTMLInputElement).value"
              class="color-picker"
            />
            <button
              class="color-reset-btn"
              @click="tempColors.fresh = ''"
              title="恢复默认"
            >↺</button>
          </div>
          <div class="color-item">
            <span class="color-label">Flow</span>
            <input
              type="color"
              :value="thresholdConfig?.flow_color || '#0EA5E9'"
              @input="(e) => tempColors.flow = (e.target as HTMLInputElement).value"
              class="color-picker"
            />
            <button
              class="color-reset-btn"
              @click="tempColors.flow = ''"
              title="恢复默认"
            >↺</button>
          </div>
          <div class="color-item">
            <span class="color-label">Warning</span>
            <input
              type="color"
              :value="thresholdConfig?.warning_color || '#F59E0B'"
              @input="(e) => tempColors.warning = (e.target as HTMLInputElement).value"
              class="color-picker"
            />
            <button
              class="color-reset-btn"
              @click="tempColors.warning = ''"
              title="恢复默认"
            >↺</button>
          </div>
          <div class="color-item">
            <span class="color-label">Panic</span>
            <input
              type="color"
              :value="thresholdConfig?.panic_color || '#F97316'"
              @input="(e) => tempColors.panic = (e.target as HTMLInputElement).value"
              class="color-picker"
            />
            <button
              class="color-reset-btn"
              @click="tempColors.panic = ''"
              title="恢复默认"
            >↺</button>
          </div>
          <div class="color-item">
            <span class="color-label">Exhausted</span>
            <input
              type="color"
              :value="thresholdConfig?.exhausted_color || '#EF4444'"
              @input="(e) => tempColors.exhausted = (e.target as HTMLInputElement).value"
              class="color-picker"
            />
            <button
              class="color-reset-btn"
              @click="tempColors.exhausted = ''"
              title="恢复默认"
            >↺</button>
          </div>
        </div>

        <div class="setting-item">
          <button class="save-btn" @click="handleSaveColors">
            保存颜色设置
          </button>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="error-bar">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4M12 16h.01"/>
        </svg>
        <span>{{ error }}</span>
      </div>
    </main>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap');

/* ── 基础容器 ── */
.settings-panel {
  width: 100%;
  height: 100%;
  font-family: 'JetBrains Mono', 'SF Mono', 'Consolas', monospace;
  background: #0a0a0b;
  color: #e4e4e7;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ── 浅色主题 ── */
.settings-panel[data-theme="light"] {
  background: #f5f5f4;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .panel-header {
  background: #ffffff;
  border-bottom-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .panel-title {
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .tab-nav {
  background: #ffffff;
  border-bottom-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .tab-btn {
  color: #737373;
}

.settings-panel[data-theme="light"] .tab-btn:hover {
  color: #52525b;
  background: #fafaf9;
}

.settings-panel[data-theme="light"] .tab-btn.active {
  color: #1c1c1e;
  border-bottom-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .setting-item {
  background: #ffffff;
  border-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .setting-label {
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .setting-desc,
.settings-panel[data-theme="light"] .model-provider {
  color: #737373;
}

.settings-panel[data-theme="light"] .model-item {
  background: #fafaf9;
  border-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .model-item.active {
  background: #ffffff;
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .select-input {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .number-input {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .pet-option,
.settings-panel[data-theme="light"] .display-option {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .pet-option.selected,
.settings-panel[data-theme="light"] .display-option.selected {
  background: #ffffff;
  border-color: #1c1c1e;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .action-btn {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .action-btn:hover {
  background: #ffffff;
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .theme-btn {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .theme-btn.active {
  background: #1c1c1e;
  border-color: #1c1c1e;
  color: #ffffff;
}

.settings-panel[data-theme="light"] .action-btn.edit-key {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .action-btn.edit-key:hover {
  background: #ffffff;
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .api-key-editor {
  background: #ffffff;
  border-top-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .api-key-input {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .api-key-input:focus {
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .api-key-hint {
  color: #a1a1aa;
}

/* ── 顶部栏 ── */
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: #111113;
  border-bottom: 1px solid #1c1c1e;
  flex-shrink: 0;
  cursor: move;
  user-select: none;
}

.panel-header:active {
  cursor: grabbing;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.panel-title {
  font-size: 11px;
  font-weight: 700;
  color: #e4e4e7;
  letter-spacing: 1px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s;
}

.icon-btn:hover {
  background: #18181b;
  color: #a1a1aa;
}

.icon-btn.close:hover {
  background: #450a0a;
  color: #f87171;
}

/* ── 标签导航 ── */
.tab-nav {
  display: flex;
  gap: 0;
  background: #111113;
  border-bottom: 1px solid #1c1c1e;
  flex-shrink: 0;
}

.tab-btn {
  padding: 12px 20px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  font-family: inherit;
  font-size: 10px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.5px;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  color: #a1a1aa;
  background: rgba(255, 255, 255, 0.03);
}

.tab-btn.active {
  color: #e4e4e7;
  border-bottom-color: #e4e4e7;
}

/* ── 内容区 ── */
.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.panel-content::-webkit-scrollbar {
  width: 4px;
}

.panel-content::-webkit-scrollbar-track {
  background: transparent;
}

.panel-content::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 2px;
}

/* ── 设置项 ── */
.settings-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  background: #111113;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  gap: 16px;
}

.setting-item.full {
  flex-direction: column;
  align-items: stretch;
  gap: 12px;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.setting-label {
  font-size: 10px;
  font-weight: 700;
  color: #e4e4e7;
  letter-spacing: 0.5px;
}

.setting-desc {
  font-size: 9px;
  color: #71717a;
}

/* ── 开关 ── */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 22px;
  flex-shrink: 0;
}

.toggle-switch.mini {
  width: 36px;
  height: 18px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-track {
  position: absolute;
  inset: 0;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  transition: all 0.2s;
}

.toggle-thumb {
  position: absolute;
  width: 16px;
  height: 16px;
  top: 2px;
  left: 2px;
  background: #71717a;
  border-radius: 3px;
  transition: all 0.2s;
}

.toggle-switch.mini .toggle-thumb {
  width: 12px;
  height: 12px;
  top: 2px;
  left: 2px;
}

.toggle-switch input:checked + .toggle-track {
  border-color: #22c55e;
}

.toggle-switch input:checked ~ .toggle-thumb {
  transform: translateX(22px);
  background: #22c55e;
}

.toggle-switch.mini input:checked ~ .toggle-thumb {
  transform: translateX(18px);
}

/* ── 主题选择器 ── */
.theme-selector {
  display: flex;
  gap: 8px;
}

.theme-btn {
  padding: 8px 16px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.theme-btn:hover {
  border-color: #3f3f46;
  color: #e4e4e7;
}

.theme-btn.active {
  background: #e4e4e7;
  border-color: #e4e4e7;
  color: #0a0a0b;
}

/* ── 模型选择器 ── */
.model-selector {
  flex-shrink: 0;
}

.select-input {
  padding: 8px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 10px;
  color: #e4e4e7;
  cursor: pointer;
  min-width: 140px;
}

.select-input:focus {
  outline: none;
  border-color: #3f3f46;
}

/* ── 模型列表 ── */
.models-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.model-item {
  display: flex;
  flex-direction: column;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  overflow: hidden;
  transition: all 0.2s;
}

.model-item.active {
  border-color: #27272a;
}

.model-item.expanded {
  border-color: #3f3f46;
}

.model-main {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  gap: 12px;
}

.model-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.model-name {
  font-size: 11px;
  font-weight: 600;
  color: #e4e4e7;
}

.model-provider {
  font-size: 9px;
  color: #52525b;
}

.model-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-btn {
  padding: 6px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.action-btn:hover:not(:disabled) {
  background: #27272a;
  border-color: #3f3f46;
  color: #e4e4e7;
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.action-btn.edit-key {
  background: #1c1c1e;
  border-color: #27272a;
  color: #a1a1aa;
}

.action-btn.edit-key:hover {
  background: #27272a;
  border-color: #3f3f46;
  color: #e4e4e7;
}

/* ── API Key 编辑器 ── */
.api-key-editor {
  padding: 14px;
  background: #111113;
  border-top: 1px solid #1c1c1e;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.api-key-input-group {
  display: flex;
  gap: 8px;
  align-items: center;
}

.api-key-input {
  flex: 1;
  padding: 8px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 10px;
  color: #e4e4e7;
  outline: none;
  transition: border-color 0.15s;
}

.api-key-input:focus {
  border-color: #3f3f46;
}

.api-key-input::placeholder {
  color: #52525b;
}

.action-btn.save {
  background: #22c55e;
  border-color: #22c55e;
  color: #ffffff;
}

.action-btn.save:hover {
  background: #16a34a;
  border-color: #16a34a;
}

.action-btn.cancel {
  background: #1c1c1e;
  border-color: #27272a;
  color: #a1a1aa;
}

.action-btn.cancel:hover {
  background: #27272a;
  border-color: #3f3f46;
  color: #e4e4e7;
}

.api-key-hint {
  font-size: 8px;
  color: #52525b;
  text-align: center;
}

/* ── 宠物选择器 ── */
.pet-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}

.pet-option {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 14px 12px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.pet-option input {
  display: none;
}

.pet-option-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.pet-label {
  font-size: 11px;
  font-weight: 700;
  color: #a1a1aa;
  letter-spacing: 0.5px;
}

.pet-desc {
  font-size: 9px;
  color: #52525b;
}

.pet-option:hover {
  border-color: #27272a;
  transform: translateY(-1px);
}

.pet-option:hover .pet-label {
  color: #e4e4e7;
}

.pet-option.selected {
  background: #111113;
  border-color: #22c55e;
  box-shadow: 0 0 12px rgba(34, 197, 94, 0.15);
}

.pet-option.selected .pet-label {
  color: #22c55e;
}

.pet-option.selected .pet-desc {
  color: #737373;
}

/* ── 配件选择器 ── */
.accessories-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.accessory-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px 8px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.accessory-option input {
  display: none;
}

.accessory-icon {
  font-size: 24px;
  margin-bottom: 4px;
}

.accessory-label {
  font-size: 10px;
  color: #a1a1aa;
}

.accessory-option:hover {
  border-color: #27272a;
  transform: translateY(-1px);
}

.accessory-option.selected {
  background: #111113;
  border-color: #22c55e;
  box-shadow: 0 0 12px rgba(34, 197, 94, 0.15);
}

.accessory-option.selected .accessory-label {
  color: #22c55e;
}

/* 浅色主题配件选择器 */
.settings-panel[data-theme="light"] .accessory-option {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .accessory-option:hover {
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .accessory-option.selected {
  background: #ffffff;
  border-color: #22c55e;
  color: #22c55e;
}

/* ── 输入框 ── */
.input-group {
  flex-shrink: 0;
}

.number-input {
  width: 70px;
  padding: 8px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 11px;
  color: #e4e4e7;
  text-align: center;
}

.number-input:focus {
  outline: none;
  border-color: #3f3f46;
}

/* ── 展示模式选择器 ── */
.display-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.display-option {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 12px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.display-option input {
  display: none;
}

.display-option:hover {
  border-color: #27272a;
  color: #e4e4e7;
}

.display-option.selected {
  background: #111113;
  border-color: #3f3f46;
  color: #e4e4e7;
}

/* ── 错误提示 ── */
.error-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: #450a0a;
  border: 1px solid #7f1d1d;
  border-radius: 6px;
  color: #fca5a5;
  font-size: 10px;
}

.error-bar svg {
  flex-shrink: 0;
}

/* ── 阈值设置 ── */
.threshold-item {
  padding: 12px 0;
  border-bottom: 1px solid #27272a;
}

.threshold-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.threshold-label {
  font-size: 11px;
  font-weight: 600;
}

.threshold-value {
  font-size: 11px;
  font-family: 'JetBrains Mono', monospace;
  background: #27272a;
  padding: 2px 6px;
  border-radius: 4px;
}

.threshold-slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: #27272a;
  outline: none;
  -webkit-appearance: none;
}

.threshold-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--slider-color, #3f3f46);
  cursor: pointer;
  transition: transform 0.15s;
}

.threshold-slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.threshold-slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--slider-color, #3f3f46);
  cursor: pointer;
  border: none;
  transition: transform 0.15s;
}

.threshold-slider::-moz-range-thumb:hover {
  transform: scale(1.2);
}

.save-btn {
  width: 100%;
  padding: 10px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.save-btn:hover {
  background: #2563eb;
}

/* ── 颜色选择器 ── */
.color-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 12px;
  padding: 12px 0;
}

.color-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.color-label {
  font-size: 10px;
  color: #a1a1aa;
}

.color-picker {
  width: 40px;
  height: 40px;
  border: 2px solid #27272a;
  border-radius: 8px;
  cursor: pointer;
  background: none;
}

.color-picker::-webkit-color-swatch-wrapper {
  padding: 0;
}

.color-picker::-webkit-color-swatch {
  border: none;
  border-radius: 6px;
}

.color-reset-btn {
  width: 24px;
  height: 24px;
  border: 1px solid #27272a;
  background: #18181b;
  color: #a1a1aa;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.color-reset-btn:hover {
  background: #27272a;
  color: #e4e4e7;
}

/* 成长系统样式 */
.growth-level-card {
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 16px;
  border: 1px solid #27272a;
}

.level-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.level-info {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.level-number {
  font-size: 24px;
  font-weight: bold;
  color: #fbbf24;
}

.level-title {
  font-size: 14px;
  color: #a1a1aa;
}

.level-xp {
  text-align: right;
}

.xp-current {
  font-size: 14px;
  color: #60a5fa;
  font-weight: 500;
}

.xp-progress-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.xp-progress-bar {
  flex: 1;
  height: 8px;
  background: #27272a;
  border-radius: 4px;
  overflow: hidden;
}

.xp-progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #fbbf24, #f59e0b);
  border-radius: 4px;
  transition: width 0.5s ease;
}

.xp-progress-text {
  font-size: 12px;
  color: #a1a1aa;
  min-width: 80px;
  text-align: right;
}

.daily-reward-card {
  background: #27272a;
  border-radius: 10px;
  padding: 14px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.reward-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.reward-percent {
  font-size: 13px;
  color: #e4e4e7;
}

.reward-xp {
  font-size: 12px;
  color: #60a5fa;
}

.claim-btn {
  padding: 8px 16px;
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.claim-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.claim-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.claim-btn.claimed {
  background: #27272a;
  color: #71717a;
}

.claim-btn.claiming {
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

.streak-display {
  font-size: 14px;
  color: #f97316;
  font-weight: 500;
}

.unlocks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 8px;
  margin-bottom: 16px;
}

.unlock-item {
  background: #27272a;
  border-radius: 8px;
  padding: 10px 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  text-align: center;
  transition: all 0.2s;
}

.unlock-item:hover {
  background: #3f3f46;
  transform: translateY(-2px);
}

.unlock-icon {
  font-size: 20px;
}

.unlock-name {
  font-size: 10px;
  color: #a1a1aa;
}

.unlock-empty {
  grid-column: 1 / -1;
  text-align: center;
  padding: 20px;
  color: #71717a;
  font-size: 13px;
}

.unlock-preview-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.unlock-preview-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  background: #1a1a1a;
  border-radius: 8px;
  border-left: 3px solid #27272a;
}

.unlock-preview-item.current {
  background: #27272a;
  border-left-color: #fbbf24;
}

.preview-level {
  min-width: 50px;
  font-size: 12px;
  font-weight: 600;
  color: #e4e4e7;
}

.preview-items {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.preview-item {
  font-size: 16px;
}
</style>
