import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface ModelConfig {
  provider: string
  name: string
  api_key: string
  enabled: boolean
}

export interface ApiConfig {
  current_model: string
  models: ModelConfig[]
}

export interface PollingConfig {
  interval_minutes: number
}

export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
}

export function useSettings() {
  const config = ref<AppConfig>({
    api_config: {
      current_model: 'bigmodel',
      models: []
    },
    polling_config: {
      interval_minutes: 1
    }
  })

  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const testResult = ref<{ success: boolean; message: string } | null>(null)

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
    const unlisten = await listen<AppConfig>('config-changed', (event) => {
      config.value = event.payload
    })
    return unlisten
  }

  return {
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
  }
}
