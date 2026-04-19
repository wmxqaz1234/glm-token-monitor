<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler } from 'chart.js'
import { invoke } from '@tauri-apps/api/core'

// 注册 Chart.js 组件
ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, Filler)

interface HistoryData {
  date: string
  tokens_used: number
  tokens_limit: number
  time_used: number
  time_limit: number
}

const chartData = ref<HistoryData[]>([])
const isLoading = ref(true)
const timeRange = ref<'7d' | '30d' | '90d'>('7d')

// 图表颜色（根据状态动态变化）
const chartColors = {
  primary: '#3B82F6',
  primaryLight: 'rgba(59, 130, 246, 0.1)',
  warning: '#F59E0B',
  warningLight: 'rgba(245, 158, 11, 0.1)',
  danger: '#EF4444',
  dangerLight: 'rgba(239, 68, 68, 0.1)',
  grid: 'rgba(148, 163, 184, 0.1)',
  text: '#94A3B8'
}

// Token 使用率图表数据
const tokenChartData = computed(() => {
  const labels = chartData.value.map(d => {
    const date = new Date(d.date)
    return `${date.getMonth() + 1}/${date.getDate()}`
  })

  const usagePercent = chartData.value.map(d =>
    Math.round((d.tokens_used / d.tokens_limit) * 100)
  )

  // 根据最高使用率选择颜色
  const maxUsage = Math.max(...usagePercent, 0)
  let color = chartColors.primary
  let bgColor = chartColors.primaryLight
  if (maxUsage > 80) {
    color = chartColors.danger
    bgColor = chartColors.dangerLight
  } else if (maxUsage > 50) {
    color = chartColors.warning
    bgColor = chartColors.warningLight
  }

  return {
    labels,
    datasets: [{
      label: 'Token 使用率 (%)',
      data: usagePercent,
      borderColor: color,
      backgroundColor: bgColor,
      fill: true,
      tension: 0.4,
      pointRadius: 3,
      pointHoverRadius: 5
    }]
  }
})

// MCP 使用率图表数据
const mcpChartData = computed(() => {
  const labels = chartData.value.map(d => {
    const date = new Date(d.date)
    return `${date.getMonth() + 1}/${date.getDate()}`
  })

  const usagePercent = chartData.value.map(d =>
    Math.round((d.time_used / d.time_limit) * 100)
  )

  const maxUsage = Math.max(...usagePercent, 0)
  let color = chartColors.primary
  let bgColor = chartColors.primaryLight
  if (maxUsage > 80) {
    color = chartColors.danger
    bgColor = chartColors.dangerLight
  } else if (maxUsage > 50) {
    color = chartColors.warning
    bgColor = chartColors.warningLight
  }

  return {
    labels,
    datasets: [{
      label: 'MCP 使用率 (%)',
      data: usagePercent,
      borderColor: color,
      backgroundColor: bgColor,
      fill: true,
      tension: 0.4,
      pointRadius: 3,
      pointHoverRadius: 5
    }]
  }
})

// Chart.js 配置
const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false
    },
    tooltip: {
      backgroundColor: 'rgba(15, 23, 42, 0.9)',
      titleColor: '#E4E4E7',
      bodyColor: '#E4E4E7',
      borderColor: 'rgba(148, 163, 184, 0.2)',
      borderWidth: 1,
      padding: 12,
      displayColors: false,
      callbacks: {
        label: (context: any) => `${context.dataset.label}: ${context.raw}%`
      }
    }
  },
  scales: {
    x: {
      grid: {
        color: chartColors.grid,
        drawBorder: false
      },
      ticks: {
        color: chartColors.text,
        font: { size: 10 }
      }
    },
    y: {
      min: 0,
      max: 100,
      grid: {
        color: chartColors.grid,
        drawBorder: false
      },
      ticks: {
        color: chartColors.text,
        font: { size: 10 },
        callback: (value: string) => `${value}%`
      }
    }
  }
}

// 加载历史数据
async function loadHistoryData() {
  try {
    isLoading.value = true
    const days = timeRange.value === '7d' ? 7 : timeRange.value === '30d' ? 30 : 90
    const data = await invoke<HistoryData[]>('get_history_data', { days })
    chartData.value = data
  } catch (err) {
    console.error('Failed to load history data:', err)
    // 如果 command 不存在，使用模拟数据
    chartData.value = generateMockData(days)
  } finally {
    isLoading.value = false
  }
}

// 生成模拟数据（用于开发/测试）
function generateMockData(days: number): HistoryData[] {
  const data: HistoryData[] = []
  const now = new Date()
  for (let i = days - 1; i >= 0; i--) {
    const date = new Date(now)
    date.setDate(date.getDate() - i)
    const tokensUsed = Math.floor(Math.random() * 50) + 20
    data.push({
      date: date.toISOString().split('T')[0],
      tokens_used: tokensUsed * 1000,
      tokens_limit: 100000,
      time_used: Math.floor(Math.random() * 100) + 50,
      time_limit: 500
    })
  }
  return data
}

// 统计数据
const stats = computed(() => {
  if (chartData.value.length === 0) return { avgTokenUsage: 0, maxTokenUsage: 0, totalTokens: 0 }

  const tokenUsages = chartData.value.map(d => (d.tokens_used / d.tokens_limit) * 100)
  const avgTokenUsage = Math.round(tokenUsages.reduce((a, b) => a + b, 0) / tokenUsages.length)
  const maxTokenUsage = Math.round(Math.max(...tokenUsages))
  const totalTokens = chartData.value.reduce((sum, d) => sum + d.tokens_used, 0)

  return { avgTokenUsage, maxTokenUsage, totalTokens }
})

// 监听时间范围变化
watch(timeRange, () => {
  loadHistoryData()
})

onMounted(() => {
  loadHistoryData()
})

// 暴露加载方法供父组件调用
defineExpose({
  refresh: loadHistoryData
})
</script>

<template>
  <div class="usage-chart-container">
    <!-- 时间范围选择 -->
    <div class="time-range-selector">
      <button
        v-for="range in [{key: '7d', label: '7天'}, {key: '30d', label: '30天'}, {key: '90d', label: '90天'}]"
        :key="range.key"
        class="range-btn"
        :class="{ active: timeRange === range.key }"
        @click="timeRange = range.key as any"
      >
        {{ range.label }}
      </button>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-cards">
      <div class="stat-card">
        <span class="stat-label">平均使用率</span>
        <span class="stat-value">{{ stats.avgTokenUsage }}%</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">最高使用率</span>
        <span class="stat-value" :class="{ high: stats.maxTokenUsage > 80 }">{{ stats.maxTokenUsage }}%</span>
      </div>
      <div class="stat-card">
        <span class="stat-label">总消耗 Token</span>
        <span class="stat-value">{{ (stats.totalTokens / 10000).toFixed(1) }}w</span>
      </div>
    </div>

    <!-- 图表区域 -->
    <div v-if="isLoading" class="chart-loading">加载中...</div>
    <div v-else-if="chartData.length > 0" class="charts-wrapper">
      <!-- Token 使用率图表 -->
      <div class="chart-item">
        <h4 class="chart-title">Token 使用率趋势</h4>
        <div class="chart-container">
          <Line :data="tokenChartData" :options="chartOptions" />
        </div>
      </div>

      <!-- MCP 使用率图表 -->
      <div class="chart-item">
        <h4 class="chart-title">MCP 额度使用率趋势</h4>
        <div class="chart-container">
          <Line :data="mcpChartData" :options="chartOptions" />
        </div>
      </div>
    </div>
    <div v-else class="chart-empty">暂无历史数据</div>
  </div>
</template>

<style scoped>
.usage-chart-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.time-range-selector {
  display: flex;
  gap: 8px;
  background: #0a0a0b;
  padding: 4px;
  border-radius: 8px;
}

.range-btn {
  flex: 1;
  padding: 8px 12px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #71717a;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.range-btn:hover {
  color: #a1a1aa;
}

.range-btn.active {
  background: #27272a;
  color: #e4e4e7;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.stat-card {
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-label {
  font-size: 10px;
  color: #71717a;
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  color: #e4e4e7;
}

.stat-value.high {
  color: #ef4444;
}

.charts-wrapper {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.chart-item {
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 12px;
  padding: 16px;
}

.chart-title {
  margin: 0 0 12px 0;
  font-size: 12px;
  font-weight: 600;
  color: #a1a1aa;
}

.chart-container {
  height: 180px;
}

.chart-loading,
.chart-empty {
  text-align: center;
  padding: 40px;
  color: #71717a;
  font-size: 13px;
}
</style>
