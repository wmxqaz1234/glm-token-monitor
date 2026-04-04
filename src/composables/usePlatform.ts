import { ref, computed, onMounted } from 'vue'

export type Platform = 'windows' | 'macos' | 'linux' | 'unknown'

export function usePlatform() {
  const platform = ref<Platform>('unknown')

  onMounted(() => {
    if (navigator.userAgent.includes('Windows')) {
      platform.value = 'windows'
    } else if (navigator.userAgent.includes('Mac')) {
      platform.value = 'macos'
    } else if (navigator.userAgent.includes('Linux')) {
      platform.value = 'linux'
    }
  })

  const isWindows = computed(() => platform.value === 'windows')
  const isMacOS = computed(() => platform.value === 'macos')

  return { platform, isWindows, isMacOS }
}
