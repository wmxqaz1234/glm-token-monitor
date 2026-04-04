import { computed, type Ref } from 'vue'

export type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

export interface UsageData {
  used: number
  total: number
}

export const COLORS: Record<PetState, string> = {
  Fresh: '#10B981',
  Flow: '#3B82F6',
  Warning: '#F59E0B',
  Panic: '#F97316',
  Dead: '#6B7280'
}

export function useUsageState(used: Ref<number>, total: Ref<number>) {
  const usagePercent = computed(() => {
    if (total.value === 0) return 0
    return (used.value / total.value) * 100
  })

  const petState = computed<PetState>(() => {
    const p = usagePercent.value
    if (p <= 30) return 'Fresh'
    if (p <= 60) return 'Flow'
    if (p <= 80) return 'Warning'
    if (p <= 95) return 'Panic'
    return 'Dead'
  })

  const stateColor = computed(() => COLORS[petState.value])

  return {
    usagePercent,
    petState,
    stateColor
  }
}
