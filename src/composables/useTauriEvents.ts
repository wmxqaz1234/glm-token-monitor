import { ref, type Ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { UsageData } from './useUsageState'

export function useTauriEvents() {
  const usageData: Ref<UsageData> = ref({ used: 0, total: 100 })

  // 监听 Rust 端推送的 usage-update 事件
  const setupEventListener = async () => {
    const unlisten = await listen<UsageData>('usage-update', (event) => {
      usageData.value = event.payload
    })
    return unlisten
  }

  return {
    usageData,
    setupEventListener
  }
}
