import { ref, computed, watch } from 'vue'
import type { PetState } from './useUsageState'

// 对话类型
export interface Dialogue {
  id: string
  text: string
  mood: 'happy' | 'neutral' | 'sad' | 'worried' | 'excited'
  duration?: number // 显示持续时间（毫秒）
}

// 心情等级
export type MoodLevel = 'ecstatic' | 'happy' | 'neutral' | 'sad' | 'depressed'

// 心情值范围 0-100
// 80-100: ecstatic (欣喜若狂)
// 60-79: happy (开心)
// 40-59: neutral (平静)
// 20-39: sad (难过)
// 0-19: depressed (抑郁)

// 基于宠物状态的对话库
const DIALOGUES: Record<PetState, Dialogue[]> = {
  'Fresh': [
    { id: 'fresh-1', text: '今天状态真好！', mood: 'happy' },
    { id: 'fresh-2', text: '充满活力~', mood: 'excited' },
    { id: 'fresh-3', text: '来工作吧！', mood: 'happy' },
    { id: 'fresh-4', text: '感觉轻盈无比', mood: 'happy' },
    { id: 'fresh-5', text: '一切都很完美', mood: 'ecstatic' },
  ],
  'Flow': [
    { id: 'flow-1', text: '保持这个节奏', mood: 'neutral' },
    { id: 'flow-2', text: '工作顺利中...', mood: 'neutral' },
    { id: 'flow-3', text: '状态不错', mood: 'happy' },
    { id: 'flow-4', text: '继续加油', mood: 'neutral' },
    { id: 'flow-5', text: '渐入佳境', mood: 'happy' },
  ],
  'Warning': [
    { id: 'warning-1', text: '注意用量哦', mood: 'worried' },
    { id: 'warning-2', text: '额度消耗中...', mood: 'neutral' },
    { id: 'warning-3', text: '要节省一点', mood: 'worried' },
    { id: 'warning-4', text: '有点担心了', mood: 'sad' },
    { id: 'warning-5', text: '请合理使用', mood: 'neutral' },
  ],
  'Panic': [
    { id: 'panic-1', text: '额度告急！', mood: 'worried' },
    { id: 'panic-2', text: '好紧张...', mood: 'sad' },
    { id: 'panic-3', text: '要不够用了', mood: 'worried' },
    { id: 'panic-4', text: '快省着点用', mood: 'sad' },
    { id: 'panic-5', text: '救命啊...', mood: 'depressed' },
  ],
  'Exhausted': [
    { id: 'exhausted-1', text: '几乎没了...', mood: 'depressed' },
    { id: 'exhausted-2', text: '快耗尽了', mood: 'sad' },
    { id: 'exhausted-3', text: '呜呜呜...', mood: 'sad' },
    { id: 'exhausted-4', text: '要休息了', mood: 'sad' },
    { id: 'exhausted-5', text: '能量不足', mood: 'depressed' },
  ],
  'Dead': [
    { id: 'dead-1', text: '已经耗尽...', mood: 'depressed' },
    { id: 'dead-2', text: '等待刷新', mood: 'sad' },
    { id: 'dead-3', text: '没力气了', mood: 'depressed' },
    { id: 'dead-4', text: 'zzZ...', mood: 'neutral' },
    { id: 'dead-5', text: '明天见...', mood: 'sad' },
  ]
}

// 特殊事件对话
const SPECIAL_DIALOGUES: Record<string, Dialogue[]> = {
  'morning': [
    { id: 'morning-1', text: '早上好！', mood: 'happy' },
    { id: 'morning-2', text: '新的一天开始', mood: 'excited' },
  ],
  'afternoon': [
    { id: 'afternoon-1', text: '下午好~', mood: 'neutral' },
    { id: 'afternoon-2', text: '继续加油', mood: 'happy' },
  ],
  'evening': [
    { id: 'evening-1', text: '晚上好', mood: 'neutral' },
    { id: 'evening-2', text: '辛苦了', mood: 'happy' },
  ],
  'late-night': [
    { id: 'late-1', text: '这么晚还在工作？', mood: 'worried' },
    { id: 'late-2', text: '该休息了', mood: 'sad' },
  ],
  'refresh': [
    { id: 'refresh-1', text: '刷新啦！', mood: 'excited' },
    { id: 'refresh-2', text: '又有额度了', mood: 'happy' },
  ],
  'threshold-warning': [
    { id: 'thresh-1', text: '注意！超过80%了', mood: 'worried' },
    { id: 'thresh-2', text: '小心用量', mood: 'sad' },
  ]
}

export function usePetDialogue(initialPetState: PetState = 'Fresh') {
  const currentDialogue = ref<Dialogue | null>(null)
  const dialogueVisible = ref(false)
  const petState = ref<PetState>(initialPetState)
  const moodLevel = ref<MoodLevel>('neutral')

  // 定时器引用
  let dialogueTimer: number | null = null
  let randomDialogueTimer: number | null = null

  // 根据状态计算心情等级
  function calculateMoodLevel(state: PetState): MoodLevel {
    switch (state) {
      case 'Fresh': return 'ecstatic'
      case 'Flow': return 'happy'
      case 'Warning': return 'neutral'
      case 'Panic': return 'sad'
      case 'Exhausted': return 'sad'
      case 'Dead': return 'depressed'
      default: return 'neutral'
    }
  }

  // 显示对话
  function showDialogue(dialogue: Dialogue, duration: number = 3000) {
    // 清除之前的定时器
    hideDialogue()

    currentDialogue.value = dialogue
    dialogueVisible.value = true

    // 自动隐藏
    dialogueTimer = window.setTimeout(() => {
      hideDialogue()
    }, duration)
  }

  // 隐藏对话
  function hideDialogue() {
    if (dialogueTimer) {
      clearTimeout(dialogueTimer)
      dialogueTimer = null
    }
    dialogueVisible.value = false
    currentDialogue.value = null
  }

  // 随机选择一条对话
  function getRandomDialogue(dialogues: Dialogue[]): Dialogue {
    return dialogues[Math.floor(Math.random() * dialogues.length)]
  }

  // 根据状态显示对话
  function showDialogueForState(state: PetState) {
    const dialogues = DIALOGUES[state] || DIALOGUES['Fresh']
    showDialogue(getRandomDialogue(dialogues))
  }

  // 显示特殊事件对话
  function showDialogueForEvent(event: string) {
    const dialogues = SPECIAL_DIALOGUES[event]
    if (dialogues) {
      showDialogue(getRandomDialogue(dialogues))
    }
  }

  // 检测时间相关对话
  function checkTimeBasedDialogue() {
    const hour = new Date().getHours()
    if (hour >= 5 && hour < 12) {
      return 'morning'
    } else if (hour >= 12 && hour < 18) {
      return 'afternoon'
    } else if (hour >= 18 && hour < 23) {
      return 'evening'
    } else {
      return 'late-night'
    }
  }

  // 启动随机对话
  function startRandomDialogue(intervalMinutes: number = 5) {
    stopRandomDialogue()
    randomDialogueTimer = window.setInterval(() => {
      if (!dialogueVisible.value) {
        showDialogueForState(petState.value)
      }
    }, intervalMinutes * 60 * 1000)
  }

  // 停止随机对话
  function stopRandomDialogue() {
    if (randomDialogueTimer) {
      clearInterval(randomDialogueTimer)
      randomDialogueTimer = null
    }
  }

  // 监听状态变化
  watch(petState, (newState, oldState) => {
    moodLevel.value = calculateMoodLevel(newState)

    // 状态变化时显示对话
    if (newState !== oldState) {
      // 从较差状态变好
      if (['Dead', 'Exhausted', 'Panic'].includes(oldState) &&
          ['Fresh', 'Flow', 'Warning'].includes(newState)) {
        showDialogueForEvent('refresh')
      } else {
        showDialogueForState(newState)
      }
    }
  })

  return {
    currentDialogue,
    dialogueVisible,
    moodLevel,
    petState,
    showDialogue,
    hideDialogue,
    showDialogueForState,
    showDialogueForEvent,
    startRandomDialogue,
    stopRandomDialogue,
    checkTimeBasedDialogue
  }
}
