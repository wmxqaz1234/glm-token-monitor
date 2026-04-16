<script setup lang="ts">
import { computed, ref } from 'vue'

interface Props {
  color: string          // 主色
  strokeColor: string    // 描边色
  eyeColor?: string      // 眼睛颜色（可选，默认深色）
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Exhausted' | 'Dead'
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100
})

// 皇冠配置
const crownConfig = computed(() => {
  const baseX = 32
  const baseY = 6

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
        rotate: -5,
        color: '#E6C200',
        stroke: '#DAA520',
        gemColor: '#FF6B6B',
        showGem: true,
        isRusty: false
      }
    case 'Warning':
      return {
        x: baseX - 1,
        y: baseY + 8,
        rotate: -20,
        color: '#DAA520',
        stroke: '#B8860B',
        gemColor: '#FF8C8C',
        showGem: true,
        isRusty: false
      }
    case 'Panic':
      return {
        x: baseX - 6,
        y: baseY + 26,
        rotate: -65,
        color: '#B8860B',
        stroke: '#9A7B0A',
        gemColor: '#FFA5A5',
        showGem: true,
        isRusty: false
      }
    case 'Exhausted':
      return {
        x: baseX - 8,
        y: baseY + 28,
        rotate: -82,
        color: '#8B6914',
        stroke: '#6B4E0A',
        gemColor: '#C0C0C0',
        showGem: true,
        isRusty: false
      }
    case 'Dead':
      return {
        x: baseX - 10,
        y: baseY + 28,
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
        { id: 1, type: 'heart', x: 20, y: 10, delay: 0, size: 6 },
        { id: 2, type: 'heart', x: 44, y: 15, delay: 1.5, size: 5 },
        { id: 3, type: 'heart', x: 32, y: 8, delay: 3, size: 4 }
      ]
    case 'Flow':
      return [
        { id: 1, type: 'thought', x: 32, y: 2, delay: 0, size: 8 }
      ]
    case 'Warning':
      return [
        { id: 1, type: 'sweat', x: 26, y: 22, delay: 0, size: 3 },
        { id: 2, type: 'sweat', x: 38, y: 24, delay: 1, size: 2.5 }
      ]
    case 'Panic':
      return [
        { id: 1, type: 'exclaim', x: 32, y: 0, delay: 0, size: 10 }
      ]
    case 'Exhausted':
      return [
        { id: 1, type: 'star', x: 20, y: 5, delay: 0, size: 5 },
        { id: 2, type: 'star', x: 44, y: 8, delay: 1, size: 4 }
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
  setTimeout(() => isClicked.value = false, 500)
}

function handleMouseDown() {
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

// 计算眼神偏移
const eyeOffset = computed(() => {
  if (!isHovered.value) return { x: 0, y: 0 }

  const maxOffset = 1.5
  const dx = mousePosition.value.x - 32
  const dy = mousePosition.value.y - 32

  return {
    x: Math.max(-maxOffset, Math.min(maxOffset, dx / 10)),
    y: Math.max(-maxOffset, Math.min(maxOffset, dy / 10))
  }
})
</script>

<template>
  <div
    class="ghost-container"
    :class="{ 'is-hovered': isHovered, 'is-clicked': isClicked, 'is-long-pressed': isLongPressed }"
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
      class="ghost-svg"
    >
      <g>
        <!-- 整体漂浮动画 -->
        <animateTransform
          attributeName="transform"
          type="translate"
          values="0,0; 0,-3; 0,0"
          dur="2s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />

        <!-- Flow: 左右摇摆 -->
        <animateTransform
          v-if="state === 'Flow'"
          attributeName="transform"
          type="rotate"
          values="-2 32 32; 2 32 32; -2 32 32"
          dur="1.5s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- Warning: 抖动 -->
        <animateTransform
          v-if="state === 'Warning'"
          attributeName="transform"
          type="translate"
          values="0,0; 1.5,0; -1.5,0; 0,0"
          dur="0.2s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- Panic: 快速抖动 -->
        <animateTransform
          v-if="state === 'Panic'"
          attributeName="transform"
          type="translate"
          values="0,0; 2,1; -2,-1; 0,0"
          dur="0.1s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- Exhausted: 瘫软下沉 -->
        <animateTransform
          v-if="state === 'Exhausted'"
          attributeName="transform"
          type="translate"
          values="0,0; 0,1; 0,0"
          dur="1.5s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- 身体 - 完整的幽灵形状 -->
        <path
          :fill="props.color"
          :stroke="props.strokeColor"
          stroke-width="2"
          stroke-linejoin="round"
          stroke-linecap="round"
          d="M 18 14 Q 32 12 46 14 Q 50 14 50 18 L 50 42 Q 50 46 46 46 Q 44 48 42 46 Q 40 44 38 46 Q 36 48 34 46 Q 32 44 30 46 Q 28 48 26 46 Q 24 44 22 46 Q 20 48 18 46 Q 14 46 14 42 L 14 18 Q 14 14 18 14 Z"
        >
          <!-- Fresh: 呼吸缩放 -->
          <animateTransform
            v-if="state === 'Fresh'"
            attributeName="transform"
            type="scale"
            values="1,1; 1.02,1.02; 1,1"
            dur="2.5s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
          />
          <!-- Panic: 快速脉动 -->
          <animateTransform
            v-if="state === 'Panic'"
            attributeName="transform"
            type="scale"
            values="1,1; 1.05,1.05; 1,1"
            dur="0.3s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
          />
        </path>

        <!-- 高光 -->
        <ellipse cx="24" cy="20" rx="5" ry="3.5" fill="white" opacity="0.3"/>

        <!-- 腮红 -->
        <ellipse cx="19" cy="30" rx="4" ry="2.5" :fill="props.color" opacity="0.35">
          <animate
            attributeName="opacity"
            values="0.3;0.5;0.3"
            dur="2s"
            repeatCount="indefinite"
          />
        </ellipse>
        <ellipse cx="45" cy="30" rx="4" ry="2.5" :fill="props.color" opacity="0.35">
          <animate
            attributeName="opacity"
            values="0.3;0.5;0.3"
            dur="2s"
            begin="0.2s"
            repeatCount="indefinite"
          />
        </ellipse>

        <!-- 左眼 -->
        <g class="eye-group" :transform="`translate(${eyeOffset.x}, ${eyeOffset.y})`">
          <ellipse cx="24" cy="26" rx="6" ry="6.5" :fill="props.eyeColor">
            <!-- Fresh: 眨眼 -->
            <animate
              v-if="state === 'Fresh'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="6s"
              repeatCount="indefinite"
            />
            <!-- Flow: 眨眼 -->
            <animate
              v-if="state === 'Flow'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="5s"
              repeatCount="indefinite"
            />
            <!-- Warning: 眼睛放大 -->
            <animate
              v-if="state === 'Warning'"
              attributeName="rx"
              values="6;7;6"
              dur="2s"
              repeatCount="indefinite"
            />
            <!-- Panic: 眼睛瞪大且快速眨眼 -->
            <animate
              v-if="state === 'Panic'"
              attributeName="ry"
              values="7;1;7"
              dur="0.8s"
              repeatCount="indefinite"
            />
            <!-- Exhausted: 眼睛半闭，疲惫 -->
            <animate
              v-if="state === 'Exhausted'"
              attributeName="ry"
              values="6.5;3;6.5"
              dur="4s"
              repeatCount="indefinite"
            />
          </ellipse>
          <!-- 眼神高光 -->
          <circle cx="22" cy="24" r="2" fill="white" opacity="0.85">
            <animate
              v-if="state === 'Panic'"
              attributeName="opacity"
              values="0.85;0.3;0.85"
              dur="0.4s"
              repeatCount="indefinite"
            />
          </circle>
        </g>

        <!-- 黑眼圈（Exhausted 状态） -->
        <g v-if="state === 'Exhausted'" class="dark-circles">
          <ellipse cx="24" cy="30" rx="7" ry="2.5" fill="rgba(0,0,0,0.2)" />
          <ellipse cx="40" cy="30" rx="7" ry="2.5" fill="rgba(0,0,0,0.2)" />
        </g>

        <!-- 右眼 -->
        <g class="eye-group" :transform="`translate(${eyeOffset.x}, ${eyeOffset.y})`">
          <ellipse cx="40" cy="26" rx="6" ry="6.5" :fill="props.eyeColor">
            <!-- Fresh: 眨眼（延迟） -->
            <animate
              v-if="state === 'Fresh'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="6s"
              begin="0.15s"
              repeatCount="indefinite"
            />
            <!-- Flow: 眨眼（延迟） -->
            <animate
              v-if="state === 'Flow'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="5s"
              begin="0.15s"
              repeatCount="indefinite"
            />
            <!-- Warning: 眼睛放大 -->
            <animate
              v-if="state === 'Warning'"
              attributeName="rx"
              values="6;7;6"
              dur="2s"
              repeatCount="indefinite"
            />
            <!-- Panic: 眼睛瞪大且快速眨眼 -->
            <animate
              v-if="state === 'Panic'"
              attributeName="ry"
              values="7;1;7"
              dur="0.8s"
              begin="0.1s"
              repeatCount="indefinite"
            />
            <!-- Exhausted: 眼睛半闭，疲惫 -->
            <animate
              v-if="state === 'Exhausted'"
              attributeName="ry"
              values="6.5;3;6.5"
              dur="4s"
              begin="0.2s"
              repeatCount="indefinite"
            />
          </ellipse>
          <!-- 眼神高光 -->
          <circle cx="38" cy="24" r="2" fill="white" opacity="0.85">
            <animate
              v-if="state === 'Panic'"
              attributeName="opacity"
              values="0.85;0.3;0.85"
              dur="0.4s"
              begin="0.1s"
              repeatCount="indefinite"
            />
          </circle>
        </g>

        <!-- 小嘴巴 -->
        <path :stroke="props.strokeColor" stroke-width="1.5" fill="none" stroke-linecap="round"
          v-if="state === 'Fresh' || state === 'Flow'"
          d="M 29 34 Q 32 37 35 34"
        >
          <animate
            attributeName="d"
            values="M 29 34 Q 32 37 35 34; M 29 35 Q 32 38 35 35; M 29 34 Q 32 37 35 34"
            dur="2s"
            repeatCount="indefinite"
          />
        </path>

        <!-- Warning: 担心嘴型 -->
        <path :stroke="props.strokeColor" stroke-width="1.5" fill="none" stroke-linecap="round"
          v-if="state === 'Warning'"
          d="M 29 35 Q 32 33 35 35"
        />

        <!-- Panic: 惊恐嘴型 -->
        <ellipse :fill="props.strokeColor"
          v-if="state === 'Panic'"
          cx="32"
          cy="36"
          rx="3.5"
          ry="2.5"
        >
          <animate
            attributeName="ry"
            values="2.5;3.5;2.5"
            dur="0.2s"
            repeatCount="indefinite"
          />
        </ellipse>

        <!-- Exhausted: 波浪嘴型，疲惫 -->
        <path :stroke="props.strokeColor" stroke-width="1.5" fill="none" stroke-linecap="round"
          v-if="state === 'Exhausted'"
          d="M 28 35 Q 30 33 32 35 Q 34 37 36 35"
        />

        <!-- Dead: X_X 眼睛 -->
        <g v-if="state === 'Dead'">
          <path :stroke="props.strokeColor" stroke-width="2" stroke-linecap="round"
            d="M 20 23 L 28 31 M 28 23 L 20 31"
          />
          <path :stroke="props.strokeColor" stroke-width="2" stroke-linecap="round"
            d="M 36 23 L 44 31 M 44 23 L 36 31"
          />
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
      </g>
    </svg>
  </div>
</template>

<style scoped>
.ghost-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 5px;
}

.ghost-svg {
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
.ghost-container {
  transition: transform 0.2s ease;
}

.ghost-container.is-hovered {
  transform: scale(1.1);
}

.ghost-container.is-clicked {
  animation: bounce-click 0.5s ease;
}

.ghost-container.is-long-pressed {
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

/* 黑眼圈 */
.dark-circles ellipse {
  animation: dark-circle-pulse 3s ease-in-out infinite;
}

@keyframes dark-circle-pulse {
  0%, 100% { opacity: 0.15; }
  50% { opacity: 0.22; }
}
</style>
