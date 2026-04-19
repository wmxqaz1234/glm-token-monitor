import { ref, computed, watch } from 'vue'
import type { PetState } from './useUsageState'
import { invoke } from '@tauri-apps/api/core'

// 心情等级
export type MoodLevel = 'ecstatic' | 'happy' | 'neutral' | 'sad' | 'depressed'

// 心情值范围 0-100
// 80-100: ecstatic (欣喜若狂)
// 60-79: happy (开心)
// 40-59: neutral (平静)
// 20-39: sad (难过)
// 0-19: depressed (抑郁)

// 心情数据接口
export interface MoodData {
  value: number          // 0-100 心情值
  level: MoodLevel       // 心情等级
  energy: number         // 0-100 能量值
  affection: number      // 0-100 亲密度
  lastInteraction: number // 上次互动时间戳
}

// 默认心情数据
const DEFAULT_MOOD: MoodData = {
  value: 60,
  level: 'neutral',
  energy: 80,
  affection: 50,
  lastInteraction: Date.now()
}

// 心情配置
const MOOD_CONFIG = {
  // 心情衰减（每分钟减少）
  decayPerMinute: 0.5,
  // 互动增加心情值
  interactionBonus: 5,
  // 最大心情值
  maxValue: 100,
  // 最小心情值
  minValue: 0
}

export function usePetMood(initialState: PetState = 'Fresh') {
  const petState = ref<PetState>(initialState)
  const moodData = ref<MoodData>({ ...DEFAULT_MOOD })

  // 心情值对应的颜色
  const moodColors = {
    ecstatic: '#FBBF24',  // 金色
    happy: '#10B981',     // 绿色
    neutral: '#6B7280',   // 灰色
    sad: '#3B82F6',       // 蓝色
    depressed: '#9CA3AF'  // 暗灰色
  }

  // 根据值获取心情等级
  function getMoodLevel(value: number): MoodLevel {
    if (value >= 80) return 'ecstatic'
    if (value >= 60) return 'happy'
    if (value >= 40) return 'neutral'
    if (value >= 20) return 'sad'
    return 'depressed'
  }

  // 根据宠物状态获取心情影响值
  function getStateMoodBonus(state: PetState): number {
    switch (state) {
      case 'Fresh': return 20
      case 'Flow': return 10
      case 'Warning': return -10
      case 'Panic': return -20
      case 'Exhausted': return -30
      case 'Dead': return -40
      default: return 0
    }
  }

  // 计算心情值
  function calculateMoodValue(): number {
    const baseMood = 50 // 基础心情值
    const stateBonus = getStateMoodBonus(petState.value)
    const energyBonus = (moodData.value.energy - 50) * 0.3
    const affectionBonus = (moodData.value.affection - 50) * 0.2

    let value = baseMood + stateBonus + energyBonus + affectionBonus
    return Math.max(MOOD_CONFIG.minValue, Math.min(MOOD_CONFIG.maxValue, value))
  }

  // 心情等级（计算属性）
  const moodLevel = computed<MoodLevel>(() => {
    const value = calculateMoodValue()
    return getMoodLevel(value)
  })

  // 心情颜色
  const moodColor = computed(() => moodColors[moodLevel.value])

  // 心情描述
  const moodDescription = computed(() => {
    const descriptions = {
      ecstatic: '欣喜若狂',
      happy: '开心',
      neutral: '平静',
      sad: '难过',
      depressed: '抑郁'
    }
    return descriptions[moodLevel.value]
  })

  // 心情表情
  const moodEmoji = computed(() => {
    const emojis = {
      ecstatic: '🤩',
      happy: '😊',
      neutral: '😐',
      sad: '😢',
      depressed: '😭'
    }
    return emojis[moodLevel.value]
  })

  // 更新宠物状态
  function setPetState(state: PetState) {
    petState.value = state
    updateMoodValue()
  }

  // 更新心情值
  function updateMoodValue() {
    const newValue = calculateMoodValue()
    moodData.value.value = newValue
    moodData.value.level = getMoodLevel(newValue)
  }

  // 互动（点击宠物）
  function interact(interactionType: 'pet' | 'feed' | 'play' = 'pet') {
    const bonus = interactionType === 'pet' ? 3 : interactionType === 'feed' ? 10 : 5
    moodData.value.value = Math.min(MOOD_CONFIG.maxValue, moodData.value.value + bonus)
    moodData.value.affection = Math.min(MOOD_CONFIG.maxValue, moodData.value.affection + bonus)
    moodData.value.lastInteraction = Date.now()
    updateMoodValue()
  }

  // 时间衰减（每分钟调用）
  function decayMood() {
    // 能量自然衰减
    moodData.value.energy = Math.max(0, moodData.value.energy - 1)

    // 亲密度缓慢衰减
    moodData.value.affection = Math.max(0, moodData.value.affection - 0.2)

    // 根据状态调整衰减速度
    let decayAmount = MOOD_CONFIG.decayPerMinute
    if (['Warning', 'Panic', 'Exhausted'].includes(petState.value)) {
      decayAmount *= 2
    }
    if (petState.value === 'Dead') {
      decayAmount *= 3
    }

    moodData.value.value = Math.max(MOOD_CONFIG.minValue, moodData.value.value - decayAmount)
    updateMoodValue()
  }

  // 恢复能量（刷新后）
  function restoreEnergy() {
    moodData.value.energy = Math.min(MOOD_CONFIG.maxValue, moodData.value.energy + 30)
    updateMoodValue()
  }

  // 加载保存的心情数据
  async function loadMoodData() {
    try {
      const saved = await invoke<any>('get_growth_data')
      if (saved && saved.mood) {
        moodData.value = { ...DEFAULT_MOOD, ...saved.mood }
      }
    } catch (err) {
      console.error('Failed to load mood data:', err)
    }
  }

  // 保存心情数据
  async function saveMoodData() {
    try {
      // 这里可以扩展保存逻辑
      console.log('Mood data saved:', moodData.value)
    } catch (err) {
      console.error('Failed to save mood data:', err)
    }
  }

  // 监听状态变化
  watch(petState, () => {
    updateMoodValue()
  })

  // 启动心情衰减定时器
  let decayTimer: number | null = null

  function startDecayTimer() {
    if (decayTimer) clearInterval(decayTimer)
    decayTimer = window.setInterval(() => {
      decayMood()
    }, 60000) // 每分钟
  }

  function stopDecayTimer() {
    if (decayTimer) {
      clearInterval(decayTimer)
      decayTimer = null
    }
  }

  return {
    moodData,
    moodLevel,
    moodColor,
    moodDescription,
    moodEmoji,
    setPetState,
    interact,
    decayMood,
    restoreEnergy,
    loadMoodData,
    saveMoodData,
    startDecayTimer,
    stopDecayTimer
  }
}
