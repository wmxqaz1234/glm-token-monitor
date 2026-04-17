<script setup lang="ts">
import { computed, ref, watch } from 'vue'

interface Props {
  color: string          // 主色
  strokeColor: string    // 描边色
  eyeColor?: string      // 眼睛颜色（可选，默认深色）
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Exhausted' | 'Dead'
  accessories?: {
    sunglasses?: boolean
    bandage?: boolean
    bow?: boolean
    hat?: 'cap' | 'beanie' | 'straw_hat' | null
  }
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100,
  accessories: () => ({})
})

// 状态变化动画
const isStateTransitioning = ref(false)
const previousState = ref<typeof props.state | null>(null)

watch(() => props.state, (newState, oldState) => {
  if (oldState && oldState !== newState) {
    previousState.value = oldState
    isStateTransitioning.value = true
    setTimeout(() => {
      isStateTransitioning.value = false
    }, 500)
  }
})

// 状态转换动画类名
const stateTransitionClass = computed(() => {
  if (!isStateTransitioning.value) return ''
  const from = previousState.value?.toLowerCase()
  const to = props.state?.toLowerCase()
  return `transition-${from}-to-${to}`
})

// 皇冠配置
const crownConfig = computed(() => {
  const baseX = 32 // 宠物中心 X
  const baseY = 8  // 头顶位置

  switch (props.state) {
    case 'Fresh':
      return {
        x: baseX,
        y: baseY,
        rotate: 0,
        color: '#FFD700',
        stroke: '#FFA500',
        gemColor: '#FF6B6B',
        showGem: true,
        isRusty: false
      }
    case 'Flow':
      return {
        x: baseX,
        y: baseY,
        rotate: -8,
        color: '#E6C200',
        stroke: '#DAA520',
        gemColor: '#FF6B6B',
        showGem: true,
        isRusty: false
      }
    case 'Warning':
      return {
        x: baseX - 2,
        y: baseY + 10,
        rotate: -25,
        color: '#DAA520',
        stroke: '#B8860B',
        gemColor: '#FF8C8C',
        showGem: true,
        isRusty: false
      }
    case 'Panic':
      return {
        x: baseX - 8,
        y: baseY + 30,
        rotate: -70,
        color: '#B8860B',
        stroke: '#9A7B0A',
        gemColor: '#FFA5A5',
        showGem: true,
        isRusty: false
      }
    case 'Exhausted':
      return {
        x: baseX - 10,
        y: baseY + 32,
        rotate: -85,
        color: '#8B6914',
        stroke: '#6B4E0A',
        gemColor: '#C0C0C0',
        showGem: true,
        isRusty: false
      }
    case 'Dead':
      return {
        x: baseX - 12,
        y: baseY + 32,
        rotate: -90,
        color: '#8B4513',
        stroke: '#654321',
        gemColor: '#4A3728',
        showGem: false,
        isRusty: true
      }
    default:
      return {
        x: baseX,
        y: baseY,
        rotate: 0,
        color: '#FFD700',
        stroke: '#FFA500',
        gemColor: '#FF6B6B',
        showGem: true,
        isRusty: false
      }
  }
})

// 粒子配置
const particles = computed(() => {
  switch (props.state) {
    case 'Fresh':
      return [
        { id: 1, type: 'heart', x: 8, y: 28, delay: 0, size: 6 },
        { id: 2, type: 'heart', x: 56, y: 32, delay: 1.5, size: 5 },
        { id: 3, type: 'heart', x: 6, y: 38, delay: 3, size: 4 }
      ]
    case 'Flow':
      return [
        { id: 1, type: 'thought', x: 56, y: 30, delay: 0, size: 8 }
      ]
    case 'Warning':
      return [
        { id: 1, type: 'sweat', x: 26, y: 22, delay: 0, size: 3 },
        { id: 2, type: 'sweat', x: 38, y: 24, delay: 1, size: 2.5 }
      ]
    case 'Panic':
      return [
        { id: 1, type: 'exclaim', x: 56, y: 25, delay: 0, size: 10 }
      ]
    case 'Exhausted':
      return [
        { id: 1, type: 'star', x: 6, y: 30, delay: 0, size: 5 },
        { id: 2, type: 'star', x: 58, y: 35, delay: 1, size: 4 }
      ]
    case 'Dead':
      return [
        { id: 1, type: 'soul', x: 32, y: 36, delay: 0, size: 8 }
      ]
    default:
      return []
  }
})

// 鼠标交互状态
const isHovered = ref(false)
const isClicked = ref(false)
const isLongPressed = ref(false)
const isBeingPetted = ref(false)
const petStrokeCount = ref(0)
let petStrokeTimer: number | null = null

// 鼠标位置（用于眼神跟随）
const mousePosition = ref({ x: 32, y: 32 })

// 处理鼠标交互
function handleMouseEnter() {
  isHovered.value = true
}

function handleMouseLeave() {
  isHovered.value = false
  isLongPressed.value = false
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

// 抚摸等级（基于连续抚摸次数）
const petLevel = computed(() => {
  if (petStrokeCount.value >= 5) return 'super-happy'
  if (petStrokeCount.value >= 3) return 'very-happy'
  if (petStrokeCount.value >= 1) return 'happy'
  return ''
})

function handleMouseDown() {
  // 长按检测
  setTimeout(() => {
    if (isHovered.value) {
      isLongPressed.value = true
    }
  }, 500)
}

function handleMouseUp() {
  isLongPressed.value = false
}

function handleMouseMove(event: MouseEvent) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  mousePosition.value = {
    x: ((event.clientX - rect.left) / rect.width) * 64,
    y: ((event.clientY - rect.top) / rect.height) * 64
  }
}

// 计算眼神偏移（让眼睛看向鼠标）- 优化版
const eyeOffset = computed(() => {
  const centerX = 32
  const centerY = 32
  const maxOffset = 2.5  // 增加移动范围

  // 计算鼠标相对于中心的角度和距离
  const dx = mousePosition.value.x - centerX
  const dy = mousePosition.value.y - centerY
  const distance = Math.sqrt(dx * dx + dy * dy)
  const angle = Math.atan2(dy, dx)

  // 根据距离计算偏移量（距离越近偏移越小）
  const maxDistance = 50  // 鼠标影响范围
  const offsetAmount = Math.min(distance / maxDistance, 1) * maxOffset

  return {
    x: Math.cos(angle) * offsetAmount,
    y: Math.sin(angle) * offsetAmount
  }
})

// 瞳孔位置（更自然的眼神效果）
const pupilOffset = computed(() => {
  const centerX = 32
  const centerY = 32
  const maxPupilOffset = 1.2

  const dx = mousePosition.value.x - centerX
  const dy = mousePosition.value.y - centerY
  const distance = Math.sqrt(dx * dx + dy * dy)
  const angle = Math.atan2(dy, dx)

  const maxDistance = 40
  const offsetAmount = Math.min(distance / maxDistance, 1) * maxPupilOffset

  return {
    x: Math.cos(angle) * offsetAmount,
    y: Math.sin(angle) * offsetAmount
  }
})

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
    class="spirit-container"
    :class="{
      'is-hovered': isHovered,
      'is-clicked': isClicked,
      'is-long-pressed': isLongPressed,
      'is-being-petted': isBeingPetted,
      'pet-happy': petLevel === 'happy',
      'pet-very-happy': petLevel === 'very-happy',
      'pet-super-happy': petLevel === 'super-happy'
    }"
    :style="{ width: `${width}px`, height: `${height}px` }"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
    @click="handleClick"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @mousemove="handleMouseMove"
  >
    <svg
      :width="width"
      :height="height"
      viewBox="0 0 64 64"
      xmlns="http://www.w3.org/2000/svg"
      class="spirit-svg"
    >
      <!-- 状态动画组 -->
      <g>
        <!-- Flow: 摇摆动画 -->
        <animateTransform
          v-if="state === 'Flow'"
          attributeName="transform"
          type="rotate"
          values="-3 32 36; 3 32 36; -3 32 36"
          dur="2s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />
        <!-- Warning: 抖动动画 -->
        <animateTransform
          v-if="state === 'Warning'"
          attributeName="transform"
          type="translate"
          values="0,0; 1,0; 0,0; -1,0; 0,0"
          dur="0.3s"
          repeatCount="indefinite"
        />
        <!-- Panic: 快速弹跳 -->
        <animateTransform
          v-if="state === 'Panic'"
          attributeName="transform"
          type="translate"
          values="0,0; 0,-4; 0,0"
          dur="0.25s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
        />
        <!-- 身体 - 完整的椭圆形状，带渐变和阴影 -->
        <defs>
          <!-- 主身体渐变 -->
          <radialGradient :id="`bodyGradient-${state}`" cx="35%" cy="30%" r="70%">
            <stop offset="0%" :stop-color="lightenColor(props.color, 30)" />
            <stop offset="50%" :stop-color="props.color" />
            <stop offset="100%" :stop-color="darkenColor(props.color, 20)" />
          </radialGradient>
          <!-- 内部光泽 -->
          <radialGradient id="innerGlow" cx="30%" cy="25%" r="50%">
            <stop offset="0%" stop-color="white" stop-opacity="0.4" />
            <stop offset="100%" stop-color="white" stop-opacity="0" />
          </radialGradient>
          <!-- 阴影滤镜 -->
          <filter id="softShadow" x="-50%" y="-50%" width="200%" height="200%">
            <feDropShadow dx="0" dy="2" stdDeviation="3" flood-color="rgba(0,0,0,0.25)" />
            <feDropShadow dx="0" dy="0" stdDeviation="6" flood-color="currentColor" flood-opacity="0.15" />
          </filter>
        </defs>

        <!-- 身体阴影层 -->
        <ellipse
          cx="32"
          cy="38"
          rx="28"
          ry="28"
          fill="rgba(0,0,0,0.15)"
          filter="url(#softShadow)"
        />

        <!-- 身体主体 -->
        <ellipse
          cx="32"
          cy="36"
          rx="28"
          ry="28"
          :fill="`url(#bodyGradient-${state})`"
          :stroke="props.strokeColor"
          stroke-width="2"
          :color="props.color"
        >
          <!-- Fresh: 呼吸动画 -->
          <animateTransform
            v-if="state === 'Fresh'"
            attributeName="transform"
            type="scale"
            values="1,1; 1,1.05; 1,1"
            dur="3s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
          />
        </ellipse>
        <!-- 高光 -->
        <ellipse cx="26" cy="20" rx="6" ry="4" fill="white" opacity="0.3"/>

        <!-- 左眼（优化版：眼白+瞳孔+高光+跟随） -->
        <g class="eye-left">
          <!-- 眼白 -->
          <ellipse cx="24" cy="32" rx="4" ry="4.5" fill="white" opacity="0.9"/>
          <!-- 瞳孔（带跟随） -->
          <g :transform="`translate(${pupilOffset.x}, ${pupilOffset.y})`">
            <circle cx="24" cy="32" r="2.2" :fill="props.eyeColor">
              <!-- 平滑过渡 -->
              <animateTransform
                attributeName="transform"
                type="translate"
                :from="`${-pupilOffset.x} ${-pupilOffset.y}`"
                :to="`0 0`"
                dur="0.2s"
                fill="freeze"
              />
              <!-- Fresh: 慢眨眼 -->
              <animate
                v-if="state === 'Fresh'"
                attributeName="ry"
                values="4.5;0.5;4.5"
                dur="4s"
                repeatCount="indefinite"
              />
              <!-- Flow: 正常眨眼 -->
              <animate
                v-if="state === 'Flow'"
                attributeName="ry"
                values="4.5;0.5;4.5"
                dur="3s"
                repeatCount="indefinite"
              />
              <!-- Warning: 眼睛放大 -->
              <animate
                v-if="state === 'Warning'"
                attributeName="r"
                values="2.2;2.8;2.2"
                dur="1.5s"
                repeatCount="indefinite"
              />
              <!-- Panic: 眼睛瞪大 -->
              <animate
                v-if="state === 'Panic'"
                attributeName="r"
                values="2.2;3.2;2.2"
                dur="0.8s"
                repeatCount="indefinite"
              />
              <!-- Exhausted: 眼睛半闭 -->
              <animate
                v-if="state === 'Exhausted'"
                attributeName="ry"
                values="4.5;2;4.5"
                dur="5s"
                repeatCount="indefinite"
              />
            </circle>
            <!-- 瞳孔高光 -->
            <circle cx="23" cy="31" r="0.7" fill="white" opacity="0.85"/>
          </g>
        </g>

        <!-- 右眼（优化版：眼白+瞳孔+高光+跟随） -->
        <g class="eye-right">
          <!-- 眼白 -->
          <ellipse cx="40" cy="32" rx="4" ry="4.5" fill="white" opacity="0.9"/>
          <!-- 瞳孔（带跟随） -->
          <g :transform="`translate(${pupilOffset.x}, ${pupilOffset.y})`">
            <circle cx="40" cy="32" r="2.2" :fill="props.eyeColor">
              <!-- 平滑过渡 -->
              <animateTransform
                attributeName="transform"
                type="translate"
                :from="`${-pupilOffset.x} ${-pupilOffset.y}`"
                :to="`0 0`"
                dur="0.2s"
                fill="freeze"
              />
              <!-- Fresh: 慢眨眼（稍延迟） -->
              <animate
                v-if="state === 'Fresh'"
                attributeName="ry"
                values="4.5;0.5;4.5"
                dur="4s"
                begin="0.1s"
                repeatCount="indefinite"
              />
              <!-- Flow: 正常眨眼（稍延迟） -->
              <animate
                v-if="state === 'Flow'"
                attributeName="ry"
                values="4.5;0.5;4.5"
                dur="3s"
                begin="0.1s"
                repeatCount="indefinite"
              />
              <!-- Warning: 眼睛放大 -->
              <animate
                v-if="state === 'Warning'"
                attributeName="r"
                values="2.2;2.8;2.2"
                dur="1.5s"
                begin="0.1s"
                repeatCount="indefinite"
              />
              <!-- Panic: 眼睛瞪大 -->
              <animate
                v-if="state === 'Panic'"
                attributeName="r"
                values="2.2;3.2;2.2"
                dur="0.8s"
                begin="0.1s"
                repeatCount="indefinite"
              />
              <!-- Exhausted: 眼睛半闭 -->
              <animate
                v-if="state === 'Exhausted'"
                attributeName="ry"
                values="4.5;2;4.5"
                dur="5s"
                begin="0.2s"
                repeatCount="indefinite"
              />
            </circle>
            <!-- 瞳孔高光 -->
            <circle cx="39" cy="31" r="0.7" fill="white" opacity="0.85"/>
          </g>
        </g>

        <!-- 黑眼圈（Exhausted 状态） -->
        <g v-if="state === 'Exhausted'" class="dark-circles">
          <ellipse cx="24" cy="34" rx="4" ry="2" fill="rgba(0,0,0,0.15)" />
          <ellipse cx="40" cy="34" rx="4" ry="2" fill="rgba(0,0,0,0.15)" />
        </g>

        <!-- 嘴巴 - 根据状态动态变化 -->
        <g class="mouth">
          <!-- Fresh: 微笑 - 轻柔的弧线 -->
          <path v-if="state === 'Fresh'"
                d="M 27 44 Q 32 48 37 44"
                :stroke="props.strokeColor"
                stroke-width="2"
                stroke-linecap="round"
                fill="none"
                class="mouth-smile">
            <animate attributeName="d"
                     values="M 27 44 Q 32 48 37 44;M 27 44 Q 32 49 37 44;M 27 44 Q 32 48 37 44"
                     dur="2s"
                     repeatCount="indefinite" />
          </path>

          <!-- Flow: 淡定 - 直线微弧 -->
          <path v-if="state === 'Flow'"
                d="M 27 45 Q 32 46 37 45"
                :stroke="props.strokeColor"
                stroke-width="2"
                stroke-linecap="round"
                fill="none"
                class="mouth-neutral">
            <animate attributeName="d"
                     values="M 27 45 Q 32 46 37 45;M 27 45 Q 32 45 37 45;M 27 45 Q 32 46 37 45"
                     dur="1.5s"
                     repeatCount="indefinite" />
          </path>

          <!-- Warning: 担忧 - 波浪线 -->
          <path v-if="state === 'Warning'"
                d="M 27 46 Q 29 44 32 46 Q 35 48 37 46"
                :stroke="props.strokeColor"
                stroke-width="2"
                stroke-linecap="round"
                fill="none"
                class="mouth-worried">
            <animate attributeName="d"
                     values="M 27 46 Q 29 44 32 46 Q 35 48 37 46;M 27 46 Q 29 45 32 47 Q 35 49 37 46;M 27 46 Q 29 44 32 46 Q 35 48 37 46"
                     dur="1s"
                     repeatCount="indefinite" />
          </path>

          <!-- Panic: 张嘴 - 椭圆 -->
          <ellipse v-if="state === 'Panic'"
                   cx="32"
                   cy="46"
                   rx="3"
                   ry="4"
                   :stroke="props.strokeColor"
                   stroke-width="2"
                   fill="none"
                   class="mouth-open">
            <animate attributeName="ry"
                     values="4;6;4"
                     dur="0.4s"
                     repeatCount="indefinite" />
            <animate attributeName="cy"
                     values="46;45;46"
                     dur="0.4s"
                     repeatCount="indefinite" />
          </ellipse>

          <!-- Exhausted: 疲惫 - 下弯弧线 -->
          <path v-if="state === 'Exhausted'"
                d="M 27 47 Q 32 44 37 47"
                :stroke="props.strokeColor"
                stroke-width="1.5"
                stroke-linecap="round"
                fill="none"
                class="mouth-tired"
                opacity="0.7">
            <animate attributeName="opacity"
                     values="0.7;0.5;0.7"
                     dur="3s"
                     repeatCount="indefinite" />
          </path>

          <!-- Dead: 死线 - 直线 -->
          <line v-if="state === 'Dead'"
                x1="27"
                y1="46"
                x2="37"
                y2="46"
                :stroke="props.strokeColor"
                stroke-width="1.5"
                stroke-linecap="round"
                class="mouth-dead"
                opacity="0.5" />
        </g>
      </g>

      <!-- 皇冠 -->
      <g
        :transform="`translate(${crownConfig.x - 10}, ${crownConfig.y - 8}) rotate(${crownConfig.rotate}, 10, 8)`"
        class="crown-group"
        :class="{ 'crown-rusty': crownConfig.isRusty }"
      >
        <!-- Fresh: 皇冠轻微浮动 -->
        <animateTransform
          v-if="state === 'Fresh'"
          attributeName="transform"
          type="translate"
          values="0,0; 0,-1; 0,0"
          dur="2s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- 皇冠主体 -->
        <path
          d="M2 12 L2 6 L6 9 L10 2 L14 8 L18 2 L22 9 L26 6 L26 12 Z"
          :fill="crownConfig.color"
          :stroke="crownConfig.stroke"
          stroke-width="1"
          stroke-linejoin="round"
          stroke-linecap="round"
          class="crown-body"
        >
          <!-- Warning/Exhausted: 皇冠摇晃 -->
          <animateTransform
            v-if="state === 'Warning' || state === 'Exhausted'"
            attributeName="transform"
            type="rotate"
          values="-2 14 8; 2 14 8; -2 14 8"
          dur="0.5s"
          repeatCount="indefinite"
          />
        </path>

        <!-- 宝石 -->
        <circle
          v-if="crownConfig.showGem"
          cx="14"
          cy="7"
          r="2.5"
          :fill="crownConfig.gemColor"
          class="crown-gem"
        >
          <!-- 宝石闪光 -->
          <animate
            v-if="state === 'Fresh'"
            attributeName="opacity"
            values="1;0.6;1"
            dur="1.5s"
            repeatCount="indefinite"
          />
        </circle>

        <!-- 锈斑（仅 Dead 状态） -->
        <g v-if="crownConfig.isRusty" class="rust-spots">
          <circle cx="5" cy="10" r="1.5" fill="#654321" opacity="0.7" class="rust-spot">
            <animate attributeName="opacity" values="0.7;0.5;0.7" dur="3s" repeatCount="indefinite" />
          </circle>
          <circle cx="18" cy="4" r="1" fill="#5D3A1A" opacity="0.6" class="rust-spot">
            <animate attributeName="opacity" values="0.6;0.4;0.6" dur="2.5s" repeatCount="indefinite" begin="0.5s" />
          </circle>
          <circle cx="24" cy="9" r="1.8" fill="#654321" opacity="0.8" class="rust-spot">
            <animate attributeName="opacity" values="0.8;0.5;0.8" dur="2.8s" repeatCount="indefinite" begin="1s" />
          </circle>
          <circle cx="12" cy="11" r="1.2" fill="#5D3A1A" opacity="0.5" class="rust-spot">
            <animate attributeName="opacity" values="0.5;0.3;0.5" dur="3.2s" repeatCount="indefinite" begin="1.5s" />
          </circle>
        </g>
      </g>

      <!-- 粒子效果 -->
      <g class="particles">
        <!-- 小心心 - Fresh -->
        <g v-for="p in particles.filter(p => p.type === 'heart')" :key="p.id"
           class="particle particle-heart">
          <text :x="p.x" :y="p.y" :font-size="p.size" fill="#FF6B9D"
                text-anchor="middle" dominant-baseline="middle">
            ❤
            <animate attributeName="y" :from="p.y" :to="p.y - 15" dur="3s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="1;0" dur="3s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="0;1;0.5" dur="0.5s"
                     begin="p.delay + 's'" />
          </text>
        </g>

        <!-- 思考泡泡 - Flow -->
        <g v-for="p in particles.filter(p => p.type === 'thought')" :key="p.id"
           class="particle particle-thought">
          <ellipse :cx="p.x" :cy="p.y" :rx="p.size" :ry="p.size * 0.8"
                   fill="rgba(255,255,255,0.9)" stroke="#40A9FF" stroke-width="1">
            <animate attributeName="cy" :from="p.y" :to="p.y - 8" dur="2s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="0;1;1;0" dur="2s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="rx" values="6;8;6" dur="1s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
          </ellipse>
          <text :x="p.x" :y="p.y + 1" font-size="6" fill="#40A9FF"
                text-anchor="middle" dominant-baseline="middle">
            💭
          </text>
        </g>

        <!-- 汗珠 - Warning -->
        <g v-for="p in particles.filter(p => p.type === 'sweat')" :key="p.id"
           class="particle particle-sweat">
          <ellipse :cx="p.x" :cy="p.y" :rx="p.size * 0.6" :ry="p.size"
                   fill="rgba(100,180,255,0.8)">
            <animate attributeName="cy" :from="p.y" :to="p.y + 12" dur="1s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="0;1;1;0" dur="1s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
          </ellipse>
        </g>

        <!-- 感叹号 - Panic -->
        <g v-for="p in particles.filter(p => p.type === 'exclaim')" :key="p.id"
           class="particle particle-exclaim">
          <text :x="p.x" :y="p.y" :font-size="p.size" fill="#FF4444"
                text-anchor="middle" font-weight="bold">
            ❗
            <animate attributeName="y" values="0;-2;0" dur="0.3s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animateTransform attributeName="transform" type="rotate"
                            values="-5 32 0; 5 32 0; -5 32 0" dur="0.2s"
                            begin="p.delay + 's'" repeatCount="indefinite" />
          </text>
        </g>

        <!-- 星星 - Exhausted -->
        <g v-for="p in particles.filter(p => p.type === 'star')" :key="p.id"
           class="particle particle-star">
          <text :x="p.x" :y="p.y" :font-size="p.size" fill="#FFD700"
                text-anchor="middle">
            ⭐
            <animateTransform attributeName="transform" type="rotate"
                            :from="`0 ${p.x} ${p.y}`" :to="`360 ${p.x} ${p.y}`"
                            dur="2s" begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="0.5;1;0.5" dur="1s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
          </text>
        </g>

        <!-- 小灵魂 - Dead -->
        <g v-for="p in particles.filter(p => p.type === 'soul')" :key="p.id"
           class="particle particle-soul">
          <text :x="p.x" :y="p.y" :font-size="p.size" fill="rgba(200,200,200,0.7)"
                text-anchor="middle" dominant-baseline="middle">
            👻
            <animate attributeName="y" :from="p.y" :to="p.y - 30" dur="4s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="opacity" values="0;0.7;0.7;0" dur="4s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
            <animate attributeName="x" values="32;30;32;34;32" dur="4s"
                     begin="p.delay + 's'" repeatCount="indefinite" />
          </text>
        </g>

        <!-- 配件渲染 -->
        <!-- 墨镜 -->
        <g v-if="props.accessories?.sunglasses" class="accessory sunglasses">
          <rect x="18" y="28" width="12" height="8" rx="2" fill="#1a1a1a" stroke="#333" stroke-width="0.5"/>
          <rect x="34" y="28" width="12" height="8" rx="2" fill="#1a1a1a" stroke="#333" stroke-width="0.5"/>
          <line x1="30" y1="30" x2="34" y2="30" stroke="#333" stroke-width="1"/>
          <line x1="18" y1="30" x2="14" y2="28" stroke="#333" stroke-width="1"/>
          <line x1="46" y1="30" x2="50" y2="28" stroke="#333" stroke-width="1"/>
        </g>

        <!-- 创口贴 -->
        <g v-if="props.accessories?.bandage" class="accessory bandage">
          <rect x="42" y="35" width="10" height="6" rx="1" fill="#f5f5dc" stroke="#ddd" stroke-width="0.3"/>
          <line x1="44" y1="35" x2="44" y2="41" stroke="#ddd" stroke-width="0.3"/>
          <line x1="47" y1="35" x2="47" y2="41" stroke="#ddd" stroke-width="0.3"/>
          <line x1="50" y1="35" x2="50" y2="41" stroke="#ddd" stroke-width="0.3"/>
        </g>

        <!-- 蝴蝶结 -->
        <g v-if="props.accessories?.bow" class="accessory bow">
          <path d="M28 15 Q24 12 20 15 Q24 18 28 15" fill="#ff69b4" stroke="#ff1493" stroke-width="0.5"/>
          <path d="M28 15 Q32 12 36 15 Q32 18 28 15" fill="#ff69b4" stroke="#ff1493" stroke-width="0.5"/>
          <circle cx="28" cy="15" r="2" fill="#ff1493"/>
        </g>

        <!-- 帽子 -->
        <g v-if="props.accessories?.hat === 'cap'" class="accessory hat-cap">
          <path d="M16 20 Q32 15 48 20 L48 18 Q32 12 16 18 Z" fill="#2196F3"/>
          <ellipse cx="32" cy="20" rx="16" ry="4" fill="#1976D2"/>
          <rect x="28" y="8" width="8" height="10" fill="#1976D2"/>
        </g>

        <g v-if="props.accessories?.hat === 'beanie'" class="accessory hat-beanie">
          <path d="M18 22 Q18 10 32 8 Q46 10 46 22" fill="#9C27B0"/>
          <ellipse cx="32" cy="22" rx="14" ry="3" fill="#7B1FA2"/>
          <circle cx="32" cy="8" r="2" fill="#9C27B0"/>
        </g>

        <g v-if="props.accessories?.hat === 'straw_hat'" class="accessory hat-straw">
          <ellipse cx="32" cy="20" rx="20" ry="6" fill="#F5DEB3" stroke="#DEB887" stroke-width="0.5"/>
          <path d="M24 20 Q32 8 40 20" fill="#F5DEB3" stroke="#DEB887" stroke-width="0.5"/>
          <path d="M26 18 Q32 10 38 18" fill="none" stroke="#DEB887" stroke-width="0.5"/>
        </g>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.spirit-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 9px;
}

.spirit-svg {
  display: block;
}

/* ── 皇冠样式 ── */
.crown-group {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.crown-body {
  transition: fill 0.5s ease, stroke 0.5s ease;
}

.crown-gem {
  filter: drop-shadow(0 0 2px rgba(255, 107, 107, 0.5));
}

/* 生锈效果 */
.crown-rusty .crown-body {
  filter: sepia(0.8) contrast(1.1);
}

.crown-rusty .rust-spot {
  animation: rust-flake-off 3s ease-in-out infinite;
}

@keyframes rust-flake-off {
  0%, 100% {
    opacity: 0.7;
    transform: scale(1) translate(0, 0);
  }
  50% {
    opacity: 0.4;
    transform: scale(0.9) translate(0.5px, 1px);
  }
}

/* ── 粒子样式 ── */
.particles {
  pointer-events: none;
}

.particle {
  transform-box: fill-box;
  transform-origin: center;
}

.particle-heart text {
  filter: drop-shadow(0 0 2px rgba(255, 107, 157, 0.5));
}

.particle-thought ellipse {
  filter: drop-shadow(0 0 3px rgba(64, 169, 255, 0.4));
}

.particle-sweat ellipse {
  filter: drop-shadow(0 0 2px rgba(100, 180, 255, 0.6));
}

.particle-exclaim text {
  filter: drop-shadow(0 0 4px rgba(255, 68, 68, 0.6));
}

.particle-star text {
  filter: drop-shadow(0 0 3px rgba(255, 215, 0, 0.6));
}

.particle-soul text {
  filter: blur(0.5px);
}

/* ── 鼠标交互样式 ── */
.spirit-container {
  transition: transform 0.2s ease;
}

.spirit-container.is-hovered {
  transform: scale(1.1);
}

.spirit-container.is-clicked {
  animation: bounce-click 0.5s ease;
}

.spirit-container.is-long-pressed {
  animation: happy-wiggle 0.5s ease infinite;
}

@keyframes bounce-click {
  0%, 100% { transform: scale(1.1) translateY(0); }
  25% { transform: scale(1.15) translateY(-8px); }
  50% { transform: scale(1.1) translateY(0); }
  75% { transform: scale(1.15) translateY(-4px); }
}

@keyframes happy-wiggle {
  0%, 100% { transform: scale(1.1) rotate(0deg); }
  25% { transform: scale(1.12) rotate(-3deg); }
  75% { transform: scale(1.12) rotate(3deg); }
}

/* ── 抚摸效果 ── */
.spirit-container.is-being-petted {
  animation: pet-shimmer 0.6s ease-out;
}

.spirit-container.pet-happy {
  filter: brightness(1.1);
}

.spirit-container.pet-very-happy {
  filter: brightness(1.15) saturate(1.2);
}

.spirit-container.pet-super-happy {
  filter: brightness(1.2) saturate(1.3);
  animation: super-happy-bounce 0.5s ease infinite;
}

@keyframes pet-shimmer {
  0% {
    filter: brightness(1);
  }
  50% {
    filter: brightness(1.3) saturate(1.2);
  }
  100% {
    filter: brightness(1.1);
  }
}

@keyframes super-happy-bounce {
  0%, 100% {
    transform: scale(1.1) translateY(0);
  }
  50% {
    transform: scale(1.15) translateY(-6px);
  }
}

/* 抚摸时的心形粒子效果 */
.spirit-container.pet-happy::after,
.spirit-container.pet-very-happy::after,
.spirit-container.pet-super-happy::after {
  content: '❤';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 16px;
  opacity: 0;
  animation: heart-pop 0.8s ease-out forwards;
  pointer-events: none;
}

.spirit-container.pet-very-happy::after {
  animation: heart-pop-double 0.8s ease-out forwards;
}

.spirit-container.pet-super-happy::after {
  content: '❤❤❤';
  animation: heart-pop-triple 1s ease-out forwards;
}

@keyframes heart-pop {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(0.5);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -250%) scale(1.2);
  }
}

@keyframes heart-pop-double {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(0.5);
  }
  50% {
    opacity: 0.8;
    transform: translate(-50%, -150%) scale(1);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -300%) scale(1.2);
  }
}

@keyframes heart-pop-triple {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(0.5) rotate(0deg);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -350%) scale(1.5) rotate(15deg);
  }
}

/* 黑眼圈 */
.dark-circles ellipse {
  animation: dark-circle-pulse 3s ease-in-out infinite;
}

@keyframes dark-circle-pulse {
  0%, 100% { opacity: 0.12; }
  50% { opacity: 0.18; }
}
</style>
