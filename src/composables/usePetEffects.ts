import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { PetGrowthData } from '../types/config'

const growthData = ref<PetGrowthData>({
  level: 1,
  current_xp: 0,
  total_xp: 0,
  today_max_percent: 0,
  today_claimed: false,
  high_usage_streak: 0,
  today_date: '',
  milestones_reached: [],
  unlocked_items: []
})

const isClaiming = ref(false)
const claimResult = ref<{ xp: number; upgraded: boolean } | null>(null)

// 等级称号
const levelTitles = [
  '', // Lv.1
  '初级',
  '常用',
  '资深',
  '高级',
  '专家',
  '大师',
  '宗师',
  '传奇',
  '永恒'
]

// 每日经验值计算
function calculateDailyXp(percent: number): number {
  if (percent >= 95) return 150 // 100 + 50 完美奖励
  if (percent >= 85) return 80
  if (percent >= 70) return 50
  if (percent >= 50) return 30
  return 10
}

// 下一级所需经验
function getNextLevelXp(level: number): number {
  const thresholds = [500, 1500, 3000, 5000, 8000, 12000, 20000, 30000, 50000]
  return thresholds[level - 1] || 50000
}

// 等级进度 (0-1)
function getLevelProgress(level: number, totalXp: number): number {
  if (level >= 10) return 1
  const thresholds = [0, 500, 1500, 3000, 5000, 8000, 12000, 20000, 30000, 50000]
  const currentThreshold = thresholds[level - 1]
  const nextThreshold = thresholds[level]
  return Math.min((totalXp - currentThreshold) / (nextThreshold - currentThreshold), 1)
}

export function usePetEffects() {
  const levelTitle = computed(() => levelTitles[growthData.value.level] || '')

  const levelProgress = computed(() =>
    getLevelProgress(growthData.value.level, growthData.value.total_xp)
  )

  const nextLevelXp = computed(() =>
    getNextLevelXp(growthData.value.level)
  )

  const currentLevelXp = computed(() => {
    if (growthData.value.level >= 10) return growthData.value.total_xp
    const thresholds = [0, 500, 1500, 3000, 5000, 8000, 12000, 20000, 30000, 50000]
    return growthData.value.total_xp - thresholds[growthData.value.level - 1]
  })

  // 检查是否解锁某个功能
  function isUnlocked(item: string): boolean {
    return growthData.value.unlocked_items.includes(item)
  }

  // 获取已解锁的特效列表
  function getUnlockedEffects(): string[] {
    return growthData.value.unlocked_items.filter(item =>
      item.startsWith('particles-') ||
      item.startsWith('aura-') ||
      item.startsWith('trail-') ||
      item.startsWith('effect-')
    )
  }

  // 获取今日可领取的经验
  function getTodayRewardXp(): number {
    if (growthData.value.today_claimed) return 0
    return calculateDailyXp(growthData.value.today_max_percent)
  }

  // 领取每日奖励
  async function claimReward(currentPercent: number): Promise<{ xp: number; upgraded: boolean }> {
    try {
      isClaiming.value = true
      const today = new Date().toISOString().split('T')[0]
      const result = await invoke<[number, boolean, PetGrowthData]>('claim_daily_growth_reward', {
        currentPercent,
        date: today
      })
      growthData.value = result[2]
      claimResult.value = { xp: result[0], upgraded: result[1] }
      return { xp: result[0], upgraded: result[1] }
    } finally {
      isClaiming.value = false
    }
  }

  // 加载成长数据
  async function loadGrowthData() {
    try {
      const data = await invoke<PetGrowthData>('get_growth_data')
      growthData.value = data
    } catch (err) {
      console.error('Failed to load growth data:', err)
    }
  }

  return {
    growthData,
    isClaiming,
    claimResult,
    levelTitle,
    levelProgress,
    nextLevelXp,
    currentLevelXp,
    isUnlocked,
    getUnlockedEffects,
    getTodayRewardXp,
    claimReward,
    loadGrowthData
  }
}
