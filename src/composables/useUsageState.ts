import { computed, type Ref } from 'vue'

export type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Exhausted' | 'Dead'

export interface UsageData {
  used: number
  total: number
  time_percent: number       // 月MCP额度 %
  tokens_percent: number     // 5h Token额度 %
  weekly_tokens_percent: number // 周Token额度 %
  time_remaining?: number    // 月MCP剩余次数（可能为null）
  tokens_reset_time?: number  // 5h Token下次重置时间（时间戳毫秒）
  weekly_tokens_reset_time?: number // 周Token下次重置时间（时间戳毫秒）
  time_reset_time?: number    // 月度额度下次重置时间（时间戳毫秒）
  level?: string             // 会员等级
  usage_details?: UsageDetail[] // 工具使用详情
  tokens_usage?: number      // 5h Token实际使用量
  weekly_tokens_usage?: number // 周 Token实际使用量
  tokens_limit?: number      // 5h Token配额总量（方案B：用于计算累积使用量）
  tokens_remaining?: number  // 5h Token剩余量（方案B）
}

export interface UsageDetail {
  model_code: string
  usage: number
}

export interface ThresholdConfig {
  fresh_threshold: number
  flow_threshold: number
  warning_threshold: number
  panic_threshold: number
  fresh_color?: string
  flow_color?: string
  warning_color?: string
  panic_color?: string
  exhausted_color?: string
}

export const DEFAULT_COLORS: Record<PetState, string> = {
  Fresh: '#1E3A8A',    // 深蓝色
  Flow: '#0EA5E9',     // 天蓝色
  Warning: '#F59E0B',
  Panic: '#F97316',
  Exhausted: '#EF4444',
  Dead: '#6B7280'
}

export function useUsageState(
  used: Ref<number>,
  total: Ref<number>,
  thresholdConfig?: Ref<ThresholdConfig | undefined>
) {
  const usagePercent = computed(() => {
    if (total.value === 0) return 0
    return (used.value / total.value) * 100
  })

  // 获取有效颜色（自定义或默认）
  const getColors = () => {
    const config = thresholdConfig?.value
    return {
      Fresh: config?.fresh_color || DEFAULT_COLORS.Fresh,
      Flow: config?.flow_color || DEFAULT_COLORS.Flow,
      Warning: config?.warning_color || DEFAULT_COLORS.Warning,
      Panic: config?.panic_color || DEFAULT_COLORS.Panic,
      Exhausted: config?.exhausted_color || DEFAULT_COLORS.Exhausted,
      Dead: DEFAULT_COLORS.Dead
    }
  }

  const colors = computed(() => getColors())

  const petState = computed<PetState>(() => {
    const p = usagePercent.value
    const config = thresholdConfig?.value

    // 使用配置的阈值，如果没有配置则使用默认值
    const fresh = config?.fresh_threshold ?? 24
    const flow = config?.flow_threshold ?? 49
    const warning = config?.warning_threshold ?? 64
    const panic = config?.panic_threshold ?? 80

    if (p <= fresh) return 'Fresh'
    if (p <= flow) return 'Flow'
    if (p <= warning) return 'Warning'
    if (p <= panic) return 'Panic'
    if (p < 100) return 'Exhausted'
    return 'Dead'
  })

  const stateColor = computed(() => colors.value[petState.value])

  const gradientColor = computed<string>(() => {
    const p = usagePercent.value
    // Dead 状态：100%及以上使用灰色
    if (p >= 100) return '#6B7280'
    // 0-100% 映射到 HSL 色相 120°(绿) → 0°(红)
    const hue = 120 * (1 - p / 100)
    return `hsl(${hue}, 75%, 45%)`
  })

  const gradientStrokeColor = computed<string>(() => {
    const p = usagePercent.value
    if (p >= 100) return '#4B5563'
    const hue = 120 * (1 - p / 100)
    return `hsl(${hue}, 80%, 35%)`
  })

  return {
    usagePercent,
    petState,
    stateColor,
    gradientColor,
    gradientStrokeColor,
    colors
  }
}
