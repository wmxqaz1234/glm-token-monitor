<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useSettings } from '../composables/useSettings'

const { usageData, setupEventListener } = useTauriEvents()
const { thresholdConfig } = useSettings()
const { usagePercent, petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total),
  thresholdConfig
)

const isOpen = ref(false)

const stateText = computed(() => {
  const stateMap = {
    Fresh: '状态良好',
    Flow: '使用中',
    Warning: '用量警告',
    Panic: '用量紧张',
    Dead: '配额耗尽'
  }
  return stateMap[petState.value] || '未知状态'
})

const remainingQuota = computed(() => {
  return usageData.value.total - usageData.value.used
})

onMounted(async () => {
  await setupEventListener()
})

function togglePanel() {
  isOpen.value = !isOpen.value
}
</script>

<template>
  <div class="popover-container">
    <!-- 触发按钮 -->
    <div
      class="trigger-button"
      :style="{ backgroundColor: stateColor }"
      @click="togglePanel"
    >
      <div class="pet-icon">
        <div class="eye left"></div>
        <div class="eye right"></div>
        <div class="mouth"></div>
      </div>
    </div>

    <!-- 气泡面板 -->
    <div class="popover-panel" :class="{ open: isOpen }">
      <div class="panel-header">
        <div class="pet-display" :style="{ backgroundColor: stateColor }">
          <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
            <div class="eye left"></div>
            <div class="eye right"></div>
            <div class="mouth"></div>
          </div>
        </div>
        <div class="header-info">
          <h3 class="app-title">glm-token-monitor</h3>
          <div class="state-badge" :style="{ backgroundColor: stateColor }">
            {{ stateText }}
          </div>
        </div>
      </div>

      <div class="panel-body">
        <!-- 使用量信息 -->
        <div class="usage-section">
          <div class="usage-label">API 配额使用情况</div>
          <div class="usage-values">
            <div class="usage-item">
              <span class="label">已使用</span>
              <span class="value">{{ usageData.used.toLocaleString() }}</span>
            </div>
            <div class="usage-item">
              <span class="label">总配额</span>
              <span class="value">{{ usageData.total.toLocaleString() }}</span>
            </div>
            <div class="usage-item">
              <span class="label">剩余</span>
              <span class="value" :class="{ low: remainingQuota < 1000 }">
                {{ remainingQuota.toLocaleString() }}
              </span>
            </div>
          </div>
        </div>

        <!-- 进度条 -->
        <div class="progress-section">
          <div class="progress-header">
            <span class="progress-label">使用率</span>
            <span class="progress-value">{{ Math.round(usagePercent) }}%</span>
          </div>
          <div class="progress-bar">
            <div
              class="progress-fill"
              :style="{ width: `${usagePercent}%`, backgroundColor: stateColor }"
            ></div>
          </div>
        </div>

        <!-- 状态说明 -->
        <div class="status-info">
          <div class="info-title">状态说明</div>
          <div class="info-content">
            <div v-if="petState === 'Fresh'" class="info-item">
              配额充足，可以正常使用
            </div>
            <div v-else-if="petState === 'Flow'" class="info-item">
              正在使用中，注意监控用量
            </div>
            <div v-else-if="petState === 'Warning'" class="info-item warning">
              配额使用过半，建议控制使用
            </div>
            <div v-else-if="petState === 'Panic'" class="info-item panic">
              配额即将耗尽，请谨慎使用
            </div>
            <div v-else-if="petState === 'Dead'" class="info-item dead">
              配额已耗尽，请及时充值
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.popover-container {
  position: relative;
  display: inline-block;
}

.trigger-button {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.trigger-button:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.pet-icon {
  position: relative;
  width: 28px;
  height: 24px;
}

.pet-icon .eye {
  position: absolute;
  width: 5px;
  height: 5px;
  background: white;
  border-radius: 50%;
  top: 6px;
}

.pet-icon .eye.left { left: 5px; }
.pet-icon .eye.right { right: 5px; }

.pet-icon .mouth {
  position: absolute;
  bottom: 5px;
  left: 50%;
  transform: translateX(-50%);
  width: 10px;
  height: 5px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 10px 10px;
}

.popover-panel {
  position: absolute;
  top: calc(100% + 12px);
  right: 0;
  width: 320px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  opacity: 0;
  visibility: hidden;
  transform: translateY(-10px);
  transition: all 0.3s ease;
  overflow: hidden;
}

.popover-panel.open {
  opacity: 1;
  visibility: visible;
  transform: translateY(0);
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
  border-bottom: 1px solid #e2e8f0;
}

.pet-display {
  width: 60px;
  height: 60px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition: background-color 0.5s ease;
}

.pet-face {
  position: relative;
  width: 40px;
  height: 32px;
}

.pet-face .eye {
  position: absolute;
  width: 6px;
  height: 6px;
  background: white;
  border-radius: 50%;
  top: 8px;
}

.pet-face .eye.left { left: 6px; }
.pet-face .eye.right { right: 6px; }

.pet-face .mouth {
  position: absolute;
  bottom: 6px;
  left: 50%;
  transform: translateX(-50%);
  width: 12px;
  height: 6px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 12px 12px;
}

.header-info {
  flex: 1;
}

.app-title {
  font-size: 18px;
  font-weight: 700;
  color: #1e293b;
  margin: 0 0 8px 0;
}

.state-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 12px;
  color: white;
  font-size: 12px;
  font-weight: 600;
  transition: background-color 0.5s ease;
}

.panel-body {
  padding: 20px;
}

.usage-section {
  margin-bottom: 20px;
}

.usage-label {
  font-size: 14px;
  font-weight: 600;
  color: #64748b;
  margin-bottom: 12px;
}

.usage-values {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.usage-item {
  flex: 1;
  background: #f8fafc;
  border-radius: 8px;
  padding: 12px;
  text-align: center;
}

.usage-item .label {
  display: block;
  font-size: 12px;
  color: #64748b;
  margin-bottom: 4px;
}

.usage-item .value {
  display: block;
  font-size: 16px;
  font-weight: 700;
  color: #1e293b;
}

.usage-item .value.low {
  color: #ef4444;
}

.progress-section {
  margin-bottom: 20px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.progress-label {
  font-size: 14px;
  font-weight: 600;
  color: #64748b;
}

.progress-value {
  font-size: 16px;
  font-weight: 700;
  color: #1e293b;
}

.progress-bar {
  height: 8px;
  background: #e2e8f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  transition: width 0.5s ease, background-color 0.5s ease;
  border-radius: 4px;
}

.status-info {
  background: #fef3c7;
  border-radius: 8px;
  padding: 12px;
}

.info-title {
  font-size: 12px;
  font-weight: 600;
  color: #92400e;
  margin-bottom: 8px;
}

.info-content {
  font-size: 13px;
  color: #78350f;
}

.info-item.warning {
  color: #d97706;
}

.info-item.panic {
  color: #dc2626;
}

.info-item.dead {
  color: #991b1b;
}

/* 状态动画 */
.state-fresh { animation: breathe 2s ease-in-out infinite; }
.state-flow { animation: type 0.8s ease-in-out infinite; }
.state-warning { animation: shake 0.5s ease-in-out infinite; }
.state-panic { animation: sweat 0.3s ease-in-out infinite; }
.state-dead { animation: ghost 2s ease-in-out infinite; }

@keyframes breathe {
  0%, 100% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}

@keyframes type {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

@keyframes sweat {
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.02) rotate(-1deg); }
  75% { transform: scale(0.98) rotate(1deg); }
}

@keyframes ghost {
  0% { transform: translateY(0); opacity: 1; }
  50% { transform: translateY(-10px); opacity: 0.5; }
  100% { transform: translateY(0); opacity: 1; }
}
</style>
