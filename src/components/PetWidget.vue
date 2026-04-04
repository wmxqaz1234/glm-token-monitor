<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { usagePercent, petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const isHovered = ref(false)

onMounted(async () => {
  await setupEventListener()
})
</script>

<template>
  <div class="pet-widget" data-tauri-drag-region>
    <div
      class="pet-content"
      @mouseenter="isHovered = true"
      @mouseleave="isHovered = false"
    >
      <div class="pet-container" :style="{ backgroundColor: stateColor }">
        <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
          <div class="eye left"></div>
          <div class="eye right"></div>
          <div class="mouth"></div>
        </div>
      </div>

      <div class="status-bar" :class="{ expanded: isHovered }">
        <div class="progress-track">
          <div class="progress-fill" :style="{ width: `${usagePercent}%`, backgroundColor: stateColor }"></div>
        </div>
        <div class="percent-text">{{ Math.round(usagePercent) }}%</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pet-widget {
  width: 150px;
  height: 150px;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.pet-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
}

.pet-container {
  width: 70px;
  height: 70px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.5s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1),
              0 0 20px var(--glow-color, rgba(16, 185, 129, 0.3));
  animation: glow-pulse 2s ease-in-out infinite;
  pointer-events: none;
  user-select: none;
}

.pet-face {
  position: relative;
  width: 44px;
  height: 36px;
  pointer-events: none;
  user-select: none;
}

.eye {
  position: absolute;
  width: 7px;
  height: 7px;
  background: white;
  border-radius: 50%;
  top: 9px;
  pointer-events: none;
}

.eye.left { left: 7px; }
.eye.right { right: 7px; }

.mouth {
  position: absolute;
  bottom: 7px;
  left: 50%;
  transform: translateX(-50%);
  width: 14px;
  height: 7px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 14px 14px;
  pointer-events: none;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 0;
  max-width: 110px;
  opacity: 0;
  overflow: hidden;
  transition: all 0.3s ease;
  pointer-events: none;
}

.status-bar.expanded {
  width: 110px;
  opacity: 1;
}

.progress-track {
  flex: 1;
  height: 5px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
  overflow: hidden;
  min-width: 0;
}

.progress-fill {
  height: 100%;
  transition: width 0.5s ease, background-color 0.5s ease;
  pointer-events: none;
}

.percent-text {
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
  pointer-events: none;
}

/* State colors and glow effects */
.state-fresh .pet-container { --glow-color: rgba(16, 185, 129, 0.4); }
.state-flow .pet-container { --glow-color: rgba(59, 130, 246, 0.4); }
.state-warning .pet-container { --glow-color: rgba(245, 158, 11, 0.4); }
.state-panic .pet-container { --glow-color: rgba(249, 115, 22, 0.5); }
.state-dead .pet-container { --glow-color: rgba(107, 114, 128, 0.2); }

/* Glow pulse animation */
@keyframes glow-pulse {
  0%, 100% { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 0 20px var(--glow-color); }
  50% { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 0 30px var(--glow-color); }
}

/* State animations */
/* Fresh 呼吸动画 */
.state-fresh {
  animation: breathe 2s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}

/* Flow 敲打动画 */
.state-flow {
  animation: type 0.8s ease-in-out infinite;
}

@keyframes type {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* Warning 抖动动画 */
.state-warning {
  animation: shake 0.5s ease-in-out infinite;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

/* Panic 冒汗动画 */
.state-panic {
  animation: sweat 0.3s ease-in-out infinite;
}

@keyframes sweat {
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.02) rotate(-1deg); }
  75% { transform: scale(0.98) rotate(1deg); }
}

/* Dead 灵魂出窍动画 */
.state-dead {
  animation: ghost 2s ease-in-out infinite;
}

@keyframes ghost {
  0% { transform: translateY(0); opacity: 1; }
  50% { transform: translateY(-10px); opacity: 0.5; }
  100% { transform: translateY(0); opacity: 1; }
}
</style>
