import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import type { AppConfig, ModelConfig, PetType, BasicConfig, ThresholdConfig } from '../types/config'

const globalConfig = ref<AppConfig>({
  api_config: {
    current_model: 'bigmodel',
    models: []
  },
  polling_config: {
    interval_minutes: 1
  },
  display_config: {
    display_mode: 'holo-bubble'
  },
  pet_config: {
    selected_pet: 'spirit' as PetType,
    action_interval: 25
  },
  basic_config: {
    enable_glow: true,
    auto_start: false,
    theme: 'dark' as 'dark' | 'light'
  },
  threshold_config: {
    fresh_threshold: 25,
    flow_threshold: 50,
    warning_threshold: 75,
    panic_threshold: 90
  }
})

const globalIsLoading = ref(false)
const globalError = ref<string | null>(null)
const globalTestResult = ref<{ success: boolean; message: string } | null>(null)
export const configEventCount = ref(0) // Exported for visual debug

export function useSettings() {
  const config = globalConfig
  const isLoading = globalIsLoading
  const error = globalError
  const testResult = globalTestResult

  // 当前激活的模型
  const activeModel = computed(() => {
    return config.value.api_config.models.find(
      m => m.provider === config.value.api_config.current_model
    )
  })

  // 可用模型列表
  const availableModels = computed(() => {
    return config.value.api_config.models.filter(m => m.enabled)
  })

  // 加载配置
  async function loadConfig() {
    try {
      isLoading.value = true
      error.value = null
      const loaded = await invoke<AppConfig>('get_config')
      console.log('[DEBUG useSettings] loadConfig response:', JSON.stringify(loaded.display_config))
      config.value = loaded
    } catch (err) {
      error.value = String(err)
      console.error('Failed to load config:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 保存配置
  async function saveConfig() {
    try {
      isLoading.value = true
      error.value = null
      await invoke('save_config_handler', { config: config.value })

      // [HOTFIX] Bypass strict Tauri Event capabilities by using native Web BroadcastChannel
      try {
        const bc = new BroadcastChannel('glm-token-monitor_config')
        bc.postMessage(JSON.parse(JSON.stringify(config.value)))
        bc.close()
      } catch (e) {
        console.error('BroadcastChannel emit failed:', e)
      }

    } catch (err) {
      error.value = String(err)
      console.error('Failed to save config:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  // 切换模型
  function switchModel(provider: string) {
    config.value.api_config.current_model = provider
  }

  // 更新模型配置
  function updateModelConfig(provider: string, updates: Partial<ModelConfig>) {
    const model = config.value.api_config.models.find(m => m.provider === provider)
    if (model) {
      Object.assign(model, updates)
    }
  }

  // 测试 API 连接
  async function testConnection(apiUrl: string, apiKey: string) {
    try {
      testResult.value = null
      const result = await invoke<string>('test_api_connection', {
        apiUrl,
        apiKey
      })
      testResult.value = { success: true, message: result }
      return result
    } catch (err) {
      testResult.value = { success: false, message: String(err) }
      throw err
    }
  }

  // 监听配置变更事件
  async function setupConfigListener() {
    console.log('[DEBUG useSettings] Adding listeners...')

    // [HOTFIX] Native Web BroadcastChannel as primary fallback!
    const bcListener = new BroadcastChannel('glm-token-monitor_config')
    bcListener.onmessage = async (event) => {
      if (event.data) {
        Object.assign(config.value, event.data)
      }
      await loadConfig()
    }

    const unlisten = await listen<AppConfig>('config-changed', async (event) => {
      if (event.payload) {
        Object.assign(config.value, event.payload)
      }
      await loadConfig()
    })

    return async () => {
      bcListener.close()
      await unlisten()
    }
  }

  // 宠物配置
  const petConfig = computed(() => config.value?.pet_config)

  // 更新宠物类型
  async function updatePetType(petType: string) {
    if (config.value?.pet_config) {
      config.value.pet_config.selected_pet = petType
    }
  }

  // 更新动作间隔
  async function updateActionInterval(interval: number) {
    if (config.value?.pet_config) {
      config.value.pet_config.action_interval = interval
    }
  }

  // 更新宠物配件
  async function updatePetAccessories(key: string, value: boolean | string | null) {
    if (!config.value?.pet_config) {
      config.value.pet_config = { selected_pet: 'spirit', action_interval: 25, accessories: {} }
    }
    if (!config.value.pet_config.accessories) {
      config.value.pet_config.accessories = {}
    }

    // 处理帽子选择（互斥）
    if (key === 'hat') {
      if (value === null) {
        config.value.pet_config.accessories.hat = null
      } else {
        // 取消其他帽子选择
        config.value.pet_config.accessories.hat = value as 'cap' | 'beanie' | 'straw_hat'
      }
    } else {
      // 布尔值配件（墨镜、创口贴、蝴蝶结）
      config.value.pet_config.accessories[key as 'sunglasses' | 'bandage' | 'bow'] = value as boolean
    }
  }

  // 基础配置
  const basicConfig = computed(() => config.value?.basic_config)

  // 更新光晕层开关
  async function updateEnableGlow(enabled: boolean) {
    if (config.value?.basic_config) {
      config.value.basic_config.enable_glow = enabled
    }
  }

  // 更新开机自启动
  async function updateAutoStart(enabled: boolean) {
    if (config.value?.basic_config) {
      config.value.basic_config.auto_start = enabled
      // 调用 Rust 端设置开机自启动
      try {
        await invoke('set_auto_start', { enabled })
      } catch (err) {
        console.error('Failed to set auto start:', err)
        throw err
      }
    }
  }

  // 检查是否配置了 API Key
  const hasApiKey = computed(() => {
    const key = activeModel.value?.api_key || ''
    return key && key.trim().length > 0
  })

  // 阈值配置
  const thresholdConfig = computed(() => config.value?.threshold_config)

  // 更新阈值配置
  async function updateThresholds(thresholds: {
    fresh_threshold: number
    flow_threshold: number
    warning_threshold: number
    panic_threshold: number
  }) {
    try {
      await invoke('update_threshold_config', thresholds)
      // 重新加载配置
      await loadConfig()
    } catch (err) {
      error.value = String(err)
      console.error('Failed to update thresholds:', err)
      throw err
    }
  }

  // 更新颜色配置
  async function updateColors(colors: {
    fresh_color?: string
    flow_color?: string
    warning_color?: string
    panic_color?: string
    exhausted_color?: string
  }) {
    try {
      await invoke('update_color_config', colors)
      // 重新加载配置
      await loadConfig()
    } catch (err) {
      error.value = String(err)
      console.error('Failed to update colors:', err)
      throw err
    }
  }

  return {
    config,
    isLoading,
    error,
    testResult,
    activeModel,
    availableModels,
    hasApiKey,
    loadConfig,
    saveConfig,
    switchModel,
    updateModelConfig,
    testConnection,
    setupConfigListener,
    petConfig,
    updatePetType,
    updateActionInterval,
    updatePetAccessories,
    basicConfig,
    updateEnableGlow,
    updateAutoStart,
    thresholdConfig,
    updateThresholds,
    updateColors
  }
}
