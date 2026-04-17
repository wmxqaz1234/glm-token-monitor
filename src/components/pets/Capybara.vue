<script setup lang="ts">
import { computed, ref } from 'vue'
import type { CapybaraAction } from '../types/config'

interface Props {
  color: string          // 主色（根据状态动态变化）
  strokeColor: string    // 描边色
  eyeColor?: string      // 眼睛颜色（可选，默认深色）
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Exhausted' | 'Dead'
  action?: CapybaraAction // 动作类型
  width?: number
  height?: number
  accessories?: {
    sunglasses?: boolean
    bandage?: boolean
    bow?: boolean
    hat?: 'cap' | 'beanie' | 'straw_hat' | null
  }
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#2D2420',
  width: 80,
  height: 80,
  action: 'capybara-idle',
  accessories: () => ({})
})

// 鼠标交互状态
const isHovered = ref(false)
const isClicked = ref(false)
const isBeingPetted = ref(false)
const petStrokeCount = ref(0)
let petStrokeTimer: number | null = null

// 鼠标位置（用于眼神跟随）
const mousePosition = ref({ x: 40, y: 40 })

// 处理鼠标交互
function handleMouseEnter() {
  isHovered.value = true
}

function handleMouseLeave() {
  isHovered.value = false
}

function handleClick() {
  isClicked.value = true
  isBeingPetted.value = true

  // 抚摸计数
  petStrokeCount.value++
  if (petStrokeTimer) clearTimeout(petStrokeTimer)
  petStrokeTimer = window.setTimeout(() => {
    petStrokeCount.value = 0
  }, 2000)

  // 重置状态
  setTimeout(() => {
    isClicked.value = false
    isBeingPetted.value = false
  }, 600)
}

// 抚摸等级
const petLevel = computed(() => {
  if (petStrokeCount.value >= 5) return 'super-happy'
  if (petStrokeCount.value >= 3) return 'very-happy'
  if (petStrokeCount.value >= 1) return 'happy'
  return ''
})

function handleMouseMove(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  mousePosition.value = {
    x: ((event.clientX - rect.left) / rect.width) * 80,
    y: ((event.clientY - rect.top) / rect.height) * 80
  }
}

// 眼神跟随（卡皮巴拉眼神移动范围小，体现淡定）
const eyeOffset = computed(() => {
  const centerX = 40
  const centerY = 40
  const maxOffset = 1  // 非常小的移动范围

  const dx = mousePosition.value.x - centerX
  const dy = mousePosition.value.y - centerY
  const distance = Math.sqrt(dx * dx + dy * dy)
  const angle = Math.atan2(dy, dx)

  const maxDistance = 50
  const offsetAmount = Math.min(distance / maxDistance, 1) * maxOffset

  return {
    x: Math.cos(angle) * offsetAmount,
    y: Math.sin(angle) * offsetAmount
  }
})

// 根据动作判断是否显示橘子
const showOrange = computed(() => props.action === 'capybara-orange')

// 根据动作判断是否显示温泉
const showSpa = computed(() => props.action === 'capybara-spa')

// 根据动作判断是否显示咀嚼
const isMunching = computed(() => props.action === 'capybara-munch')

// 根据动作判断是否是盯着镜头模式
const isStaring = computed(() => props.action === 'capybara-stare')

// 颜色变亮函数
function lightenColor(color: string, percent: number): string {
  const num = parseInt(color.replace('#', ''), 16)
  const amt = Math.round(2.55 * percent)
  const R = Math.min(255, (num >> 16) + amt)
  const G = Math.min(255, ((num >> 8) & 0x00FF) + amt)
  const B = Math.min(255, (num & 0x0000FF) + amt)
  return `#${(1 << 24 | R << 16 | G << 8 | B).toString(16).slice(1)}`
}

// 颜色变暗函数
function darkenColor(color: string, percent: number): string {
  const num = parseInt(color.replace('#', ''), 16)
  const amt = Math.round(2.55 * percent)
  const R = Math.max(0, (num >> 16) - amt)
  const G = Math.max(0, ((num >> 8) & 0x00FF) - amt)
  const B = Math.max(0, (num & 0x0000FF) - amt)
  return `#${(1 << 24 | R << 16 | G << 8 | B).toString(16).slice(1)}`
}
</script>

<template>
  <div
    class="capybara-container"
    :class="{
      'is-hovered': isHovered,
      'is-clicked': isClicked,
      'is-being-petted': isBeingPetted,
      'pet-happy': petLevel === 'happy',
      'pet-very-happy': petLevel === 'very-happy',
      'pet-super-happy': petLevel === 'super-happy',
      'is-staring': isStaring
    }"
    :style="{ width: `${width}px`, height: `${height}px` }"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
    @click="handleClick"
    @mousemove="handleMouseMove"
  >
    <svg :width="width" :height="height" viewBox="0 0 80 80" class="capybara-svg" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <!-- 身体渐变 -->
        <linearGradient id="capybaraBodyGradient" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" :stop-color="lightenColor(color, 15)" />
          <stop offset="50%" :stop-color="color" />
          <stop offset="100%" :stop-color="darkenColor(color, 10)" />
        </linearGradient>

        <!-- 温泉水面渐变 -->
        <radialGradient id="spaWaterGradient" cx="50%" cy="50%" r="50%">
          <stop offset="0%" stop-color="rgba(100, 180, 255, 0.5)" />
          <stop offset="100%" stop-color="rgba(100, 180, 255, 0.1)" />
        </radialGradient>

        <!-- 橘子渐变 -->
        <radialGradient id="orangeGradient" cx="30%" cy="30%">
          <stop offset="0%" stop-color="#FFAA33" />
          <stop offset="100%" stop-color="#FF8C00" />
        </radialGradient>
      </defs>

      <!-- 温泉水面（仅 spa 模式） -->
      <g v-if="showSpa" class="spa-water">
        <ellipse cx="40" cy="65" rx="30" ry="10" fill="url(#spaWaterGradient)" opacity="0.8">
          <animate attributeName="rx" values="30;31;30" dur="4s" repeatCount="indefinite" />
          <animate attributeName="ry" values="10;11;10" dur="4s" repeatCount="indefinite" />
        </ellipse>
        <!-- 水波纹 -->
        <ellipse cx="28" cy="68" rx="8" ry="3" fill="rgba(100, 180, 255, 0.3)" opacity="0">
          <animate attributeName="opacity" values="0;0.5;0" dur="3s" repeatCount="indefinite" />
          <animate attributeName="rx" values="8;14;16" dur="3s" repeatCount="indefinite" />
        </ellipse>
        <ellipse cx="52" cy="68" rx="8" ry="3" fill="rgba(100, 180, 255, 0.3)" opacity="0">
          <animate attributeName="opacity" values="0;0.5;0" dur="3s" begin="1.5s" repeatCount="indefinite" />
          <animate attributeName="rx" values="8;14;16" dur="3s" begin="1.5s" repeatCount="indefinite" />
        </ellipse>
      </g>

      <!-- 蒸汽效果（仅 spa 模式） -->
      <g v-if="showSpa" class="spa-steam">
        <path d="M 28 55 Q 26 50 28 45 Q 30 40 28 35" stroke="rgba(200, 220, 255, 0.5)" stroke-width="2.5" fill="none" opacity="0">
          <animate attributeName="opacity" values="0;0.6;0" dur="5s" repeatCount="indefinite" />
          <animate attributeName="d" values="M 28 55 Q 26 50 28 45 Q 30 40 28 35;M 28 52 Q 30 47 28 42 Q 26 37 28 32;M 28 55 Q 26 50 28 45 Q 30 40 28 35" dur="5s" repeatCount="indefinite" />
        </path>
        <path d="M 40 55 Q 38 50 40 45 Q 42 40 40 35" stroke="rgba(200, 220, 255, 0.5)" stroke-width="2.5" fill="none" opacity="0">
          <animate attributeName="opacity" values="0;0.6;0" dur="5s" begin="1.2s" repeatCount="indefinite" />
          <animate attributeName="d" values="M 40 55 Q 38 50 40 45 Q 42 40 40 35;M 40 52 Q 42 47 40 42 Q 38 37 40 32;M 40 55 Q 38 50 40 45 Q 42 40 40 35" dur="5s" begin="1.2s" repeatCount="indefinite" />
        </path>
        <path d="M 52 55 Q 50 50 52 45 Q 54 40 52 35" stroke="rgba(200, 220, 255, 0.5)" stroke-width="2.5" fill="none" opacity="0">
          <animate attributeName="opacity" values="0;0.6;0" dur="5s" begin="2.4s" repeatCount="indefinite" />
          <animate attributeName="d" values="M 52 55 Q 50 50 52 45 Q 54 40 52 35;M 52 52 Q 54 47 52 42 Q 50 37 52 32;M 52 55 Q 50 50 52 45 Q 54 40 52 35" dur="5s" begin="2.4s" repeatCount="indefinite" />
        </path>
      </g>

      <!-- 后腿（深色，在身体后面） -->
      <ellipse cx="28" cy="62" rx="8" ry="10" :fill="color" class="leg-back-left" />
      <ellipse cx="52" cy="62" rx="8" ry="10" :fill="color" class="leg-back-right" />

      <!-- 身体 - 卡皮巴拉标志性的圆润身材 -->
      <g class="capybara-body">
        <!-- 佛系呼吸动画 - 超级慢 -->
        <animateTransform
          attributeName="transform"
          type="scale"
          values="1,1; 1.008,1.008; 1,1"
          dur="6s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
        />

        <ellipse cx="40" cy="50" rx="26" ry="20" :fill="`url(#capybaraBodyGradient)`" :stroke="strokeColor" stroke-width="2" />

        <!-- 肚子（更浅的颜色） -->
        <ellipse cx="40" cy="54" rx="16" ry="8" :fill="lightenColor(color, 20)" opacity="0.5" />
      </g>

      <!-- 前腿 -->
      <ellipse cx="30" cy="60" rx="6" ry="8" :fill="color" :stroke="strokeColor" stroke-width="1.5" class="leg-front-left" />
      <ellipse cx="50" cy="60" rx="6" ry="8" :fill="color" :stroke="strokeColor" stroke-width="1.5" class="leg-front-right" />

      <!-- 脚趾（小圆点） -->
      <g class="toes">
        <circle cx="26" cy="66" r="1.5" :fill="darkenColor(color, 25)" />
        <circle cx="30" cy="66" r="1.5" :fill="darkenColor(color, 25)" />
        <circle cx="50" cy="66" r="1.5" :fill="darkenColor(color, 25)" />
        <circle cx="54" cy="66" r="1.5" :fill="darkenColor(color, 25)" />
      </g>

      <!-- 头部 -->
      <g class="capybara-head">
        <!-- 头部轻微浮动 -->
        <animateTransform
          attributeName="transform"
          type="translate"
          values="0,0; 0,-0.5; 0,0"
          dur="6s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
        />

        <!-- 头型 -->
        <ellipse cx="40" cy="40" rx="20" ry="16" :fill="`url(#capybaraBodyGradient)`" :stroke="strokeColor" stroke-width="2" />

        <!-- 嘴巴区域（更浅的颜色） -->
        <ellipse cx="40" cy="46" rx="12" ry="6" :fill="lightenColor(color, 15)" opacity="0.4" />

        <!-- 鼻子 -->
        <ellipse cx="40" cy="46" rx="3.5" ry="2.5" :fill="darkenColor(color, 30)" class="nose" />
        <!-- 鼻孔 -->
        <ellipse cx="38.5" cy="46" rx="1" ry="0.7" fill="rgba(0,0,0,0.4)" />
        <ellipse cx="41.5" cy="46" rx="1" ry="0.7" fill="rgba(0,0,0,0.4)" />

        <!-- 嚼草动画（仅 munch 模式） -->
        <g v-if="isMunching" class="munch-mouth">
          <ellipse cx="40" cy="50" rx="5" ry="2.5" :fill="darkenColor(color, 20)" :stroke="strokeColor" stroke-width="1">
            <animate attributeName="ry" values="2.5;4;2.5;4;2.5" dur="2s" repeatCount="indefinite" />
            <animate attributeName="cy" values="50;49;50;49;50" dur="2s" repeatCount="indefinite" />
          </ellipse>
        </g>

        <!-- 普通嘴巴 -->
        <path v-else
          d="M 35 49 Q 40 51 45 49"
          :stroke="strokeColor"
          stroke-width="1.5"
          stroke-linecap="round"
          fill="none"
          class="mouth"
        />

        <!-- 草（仅 munch 模式） -->
        <g v-if="isMunching" class="grass">
          <path d="M 35 56 Q 32 50 35 44" stroke="#4ADE80" stroke-width="2" fill="none" opacity="0">
            <animate attributeName="opacity" values="0;1;1;0" dur="2s" repeatCount="indefinite" />
            <animate attributeName="d" values="M 35 56 Q 32 50 35 44;M 35 52 Q 34 48 35 44;M 35 48 Q 35 46 35 44" dur="2s" repeatCount="indefinite" />
          </path>
          <path d="M 40 58 Q 37 52 40 46" stroke="#4ADE80" stroke-width="2" fill="none" opacity="0">
            <animate attributeName="opacity" values="0;1;1;0" dur="2s" begin="0.3s" repeatCount="indefinite" />
            <animate attributeName="d" values="M 40 58 Q 37 52 40 46;M 40 54 Q 39 50 40 46;M 40 50 Q 40 48 40 46" dur="2s" begin="0.3s" repeatCount="indefinite" />
          </path>
          <path d="M 45 56 Q 42 50 45 44" stroke="#4ADE80" stroke-width="2" fill="none" opacity="0">
            <animate attributeName="opacity" values="0;1;1;0" dur="2s" begin="0.6s" repeatCount="indefinite" />
            <animate attributeName="d" values="M 45 56 Q 42 50 45 44;M 45 52 Q 44 48 45 44;M 45 48 Q 45 46 45 44" dur="2s" begin="0.6s" repeatCount="indefinite" />
          </path>
        </g>

        <!-- 眼睛 - 豆豆眼，永远淡定 -->
        <g class="eyes">
          <!-- 左眼 -->
          <g class="eye-left" :transform="`translate(${eyeOffset.x}, ${eyeOffset.y})`">
            <circle cx="33" cy="40" r="3" :fill="eyeColor">
              <!-- 超慢眨眼 -->
              <animate
                attributeName="r"
                values="3;0.8;3"
                dur="7s"
                repeatCount="indefinite"
              />
            </circle>
            <!-- 眼神高光 -->
            <circle cx="31.5" cy="38.5" r="1" fill="white" opacity="0.6" />
          </g>

          <!-- 右眼 -->
          <g class="eye-right" :transform="`translate(${eyeOffset.x}, ${eyeOffset.y})`">
            <circle cx="47" cy="40" r="3" :fill="eyeColor">
              <!-- 超慢眨眼，稍延迟 -->
              <animate
                attributeName="r"
                values="3;0.8;3"
                dur="7s"
                begin="0.3s"
                repeatCount="indefinite"
              />
            </circle>
            <!-- 眼神高光 -->
            <circle cx="45.5" cy="38.5" r="1" fill="white" opacity="0.6" />
          </g>
        </g>

        <!-- 腮红 - 淡淡的粉色 -->
        <ellipse cx="28" cy="44" rx="3.5" ry="2" fill="#F9A8D4" opacity="0.25" />
        <ellipse cx="52" cy="44" rx="3.5" ry="2" fill="#F9A8D4" opacity="0.25" />

        <!-- 耳朵 - 小小的圆形 -->
        <ellipse cx="23" cy="32" rx="5" ry="4" :fill="color" :stroke="strokeColor" stroke-width="1.5" class="ear-left" />
        <ellipse cx="23" cy="32" rx="2.5" ry="2" :fill="darkenColor(color, 20)" class="ear-inner-left" />

        <ellipse cx="57" cy="32" rx="5" ry="4" :fill="color" :stroke="strokeColor" stroke-width="1.5" class="ear-right" />
        <ellipse cx="57" cy="32" rx="2.5" ry="2" :fill="darkenColor(color, 20)" class="ear-inner-right" />

        <!-- 橘子（仅 orange 模式） -->
        <g v-if="showOrange" class="orange-hat">
          <circle cx="40" cy="24" r="9" fill="#FF8C00" stroke="#EA580C" stroke-width="1.5">
            <!-- 橘子轻微摇晃 -->
            <animateTransform
              attributeName="transform"
              type="rotate"
              values="-2 40 40; 2 40 40; -2 40 40"
              dur="6s"
              repeatCount="indefinite"
            />
          </circle>
          <!-- 橘子渐变覆盖 -->
          <circle cx="40" cy="24" r="9" fill="url(#orangeGradient)">
            <animateTransform
              attributeName="transform"
              type="rotate"
              values="-2 40 40; 2 40 40; -2 40 40"
              dur="6s"
              repeatCount="indefinite"
            />
          </circle>
          <!-- 橘子叶子 -->
          <ellipse cx="40" cy="16" rx="4" ry="2" fill="#4ADE80" transform="rotate(15, 40, 16)">
            <animateTransform
              attributeName="transform"
              type="rotate"
              values="15 40 16; 20 40 16; 15 40 16"
              dur="6s"
              repeatCount="indefinite"
            />
          </ellipse>
          <!-- 橘子高光 -->
          <ellipse cx="36" cy="21" rx="2.5" ry="2" fill="white" opacity="0.25" />
        </g>

        <!-- 胡须 -->
        <g class="whiskers" :stroke="darkenColor(color, 15)" stroke-width="0.6" opacity="0.6">
          <line x1="30" y1="44" x2="22" y2="42" />
          <line x1="30" y1="46" x2="22" y2="46" />
          <line x1="30" y1="48" x2="22" y2="50" />
          <line x1="50" y1="44" x2="58" y2="42" />
          <line x1="50" y1="46" x2="58" y2="46" />
          <line x1="50" y1="48" x2="58" y2="50" />
        </g>
      </g>

      <!-- 配件渲染 -->
      <!-- 墨镜 -->
      <g v-if="props.accessories?.sunglasses" class="accessory sunglasses">
        <rect x="27" y="37" width="12" height="7" rx="2" fill="#1a1a1a" stroke="#333" stroke-width="0.5"/>
        <rect x="41" y="37" width="12" height="7" rx="2" fill="#1a1a1a" stroke="#333" stroke-width="0.5"/>
        <line x1="39" y1="39.5" x2="41" y2="39.5" stroke="#333" stroke-width="0.8"/>
        <line x1="27" y1="39.5" x2="23" y2="37" stroke="#333" stroke-width="0.8"/>
        <line x1="53" y1="39.5" x2="57" y2="37" stroke="#333" stroke-width="0.8"/>
      </g>

      <!-- 创口贴 -->
      <g v-if="props.accessories?.bandage" class="accessory bandage">
        <rect x="45" y="42" width="10" height="6" rx="1" fill="#f5f5dc" stroke="#ddd" stroke-width="0.3"/>
        <line x1="47" y1="42" x2="47" y2="48" stroke="#ddd" stroke-width="0.3"/>
        <line x1="50" y1="42" x2="50" y2="48" stroke="#ddd" stroke-width="0.3"/>
        <line x1="53" y1="42" x2="53" y2="48" stroke="#ddd" stroke-width="0.3"/>
      </g>

      <!-- 蝴蝶结 -->
      <g v-if="props.accessories?.bow" class="accessory bow">
        <path d="M36 28 Q32 24 28 28 Q32 32 36 28" fill="#ff69b4" stroke="#ff1493" stroke-width="0.5"/>
        <path d="M36 28 Q40 24 44 28 Q40 32 36 28" fill="#ff69b4" stroke="#ff1493" stroke-width="0.5"/>
        <circle cx="36" cy="28" r="2" fill="#ff1493"/>
      </g>

      <!-- 帽子 -->
      <g v-if="props.accessories?.hat === 'cap'" class="accessory hat-cap">
        <path d="M24 30 Q40 24 56 30 L56 27 Q40 20 24 27 Z" fill="#2196F3"/>
        <ellipse cx="40" cy="30" rx="16" ry="4" fill="#1976D2"/>
        <rect x="35" y="14" width="10" height="12" fill="#1976D2"/>
      </g>

      <g v-if="props.accessories?.hat === 'beanie'" class="accessory hat-beanie">
        <path d="M26 32 Q26 18 40 16 Q54 18 54 32" fill="#9C27B0"/>
        <ellipse cx="40" cy="32" rx="14" ry="3" fill="#7B1FA2"/>
        <circle cx="40" cy="16" r="2" fill="#9C27B0"/>
      </g>

      <g v-if="props.accessories?.hat === 'straw_hat'" class="accessory hat-straw">
        <ellipse cx="40" cy="28" rx="22" ry="7" fill="#F5DEB3" stroke="#DEB887" stroke-width="0.5"/>
        <path d="M30 28 Q40 14 50 28" fill="#F5DEB3" stroke="#DEB887" stroke-width="0.5"/>
        <path d="M32 26 Q40 16 48 26" fill="none" stroke="#DEB887" stroke-width="0.5"/>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.capybara-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 5px;
  transition: transform 0.3s ease;
}

.capybara-container.is-hovered {
  transform: scale(1.05);
}

.capybara-container.is-clicked {
  animation: capybara-bounce 0.6s ease;
}

@keyframes capybara-bounce {
  0%, 100% { transform: scale(1.05) translateY(0); }
  25% { transform: scale(1.08) translateY(-4px); }
  50% { transform: scale(1.05) translateY(0); }
  75% { transform: scale(1.08) translateY(-2px); }
}

/* 抚摸效果 */
.capybara-container.is-being-petted {
  animation: capybara-shimmer 0.6s ease-out;
}

.capybara-container.pet-happy {
  filter: brightness(1.05);
}

.capybara-container.pet-very-happy {
  filter: brightness(1.08) saturate(1.1);
}

.capybara-container.pet-super-happy {
  filter: brightness(1.1) saturate(1.15);
  animation: capybara-super-happy 0.8s ease infinite;
}

@keyframes capybara-shimmer {
  0% { filter: brightness(1); }
  50% { filter: brightness(1.15) saturate(1.1); }
  100% { filter: brightness(1.05); }
}

@keyframes capybara-super-happy {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3px); }
}

/* 配件样式 */
.accessory {
  pointer-events: none;
}

.sunglasses {
  filter: drop-shadow(0 0.5px 1px rgba(0,0,0,0.2));
}

.bandage {
  filter: drop-shadow(0 0.5px 0.5px rgba(0,0,0,0.15));
}

.bow {
  filter: drop-shadow(0 0.5px 1px rgba(255,20,147,0.2));
}

.hat-cap, .hat-beanie, .hat-straw {
  filter: drop-shadow(0 1px 1.5px rgba(0,0,0,0.15));
}

/* 盯着镜头模式 - 眼神更专注 */
.capybara-container.is-staring .eye-left,
.capybara-container.is-staring .eye-right {
  animation: stare-focus 10s ease-in-out infinite;
}

@keyframes stare-focus {
  0%, 90%, 100% { transform: translateX(0); }
  95% { transform: translateX(0.5px); }
}
</style>
