<template>
  <div class="history-panel">
    <div class="history-header">
      <h3>📊 使用统计</h3>
      <div class="time-filters">
        <button
          v-for="filter in timeFilters"
          :key="filter.value"
          :class="{ active: selectedFilter === filter.value }"
          @click="selectFilter(filter.value)"
        >
          {{ filter.label }}
        </button>
      </div>
    </div>

    <!-- 统计摘要 -->
    <div class="stats-summary" v-if="stats">
      <div class="stat-item">
        <span class="stat-label">平均使用率</span>
        <span class="stat-value">{{ stats.avg_tokens_percent.toFixed(1) }}%</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">峰值使用率</span>
        <span class="stat-value">{{ stats.max_tokens_percent }}%</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">记录总数</span>
        <span class="stat-value">{{ stats.total_records }}</span>
      </div>
    </div>

    <!-- 简易趋势图 -->
    <div class="trend-chart" v-if="history.length > 0">
      <svg viewBox="0 0 300 100" class="chart-svg">
        <polyline
          :points="chartPoints"
          fill="none"
          :stroke="chartColor"
          stroke-width="2"
        />
        <circle
          v-for="(point, index) in chartPointsArray"
          :key="index"
          :cx="point.x"
          :cy="point.y"
          r="3"
          :fill="chartColor"
        />
      </svg>
      <div class="chart-labels">
        <span>{{ timeRangeLabel }}</span>
        <span>现在</span>
      </div>
    </div>

    <!-- 历史记录列表 -->
    <div class="history-list">
      <div class="history-item" v-for="item in displayHistory" :key="item.id">
        <span class="history-time">{{ formatTime(item.timestamp) }}</span>
        <div class="history-values">
          <span class="history-value" :class="getStatusClass(item.tokens_percent)">
            Token: {{ item.tokens_percent }}%
          </span>
          <span class="history-value">MCP: {{ item.time_percent }}%</span>
        </div>
      </div>
      <div class="no-data" v-if="history.length === 0">
        暂无历史数据
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="history-actions">
      <button @click="exportCsv" class="btn-export">📤 导出 CSV</button>
      <button @click="cleanup" class="btn-cleanup">🗑️ 清理旧数据</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface HistoryEntry {
  id: number;
  timestamp: string;
  tokens_percent: number;
  time_percent: number;
  weekly_tokens_percent: number;
}

interface HistoryStats {
  total_records: number;
  avg_tokens_percent: number;
  max_tokens_percent: number;
  first_record: string | null;
  last_record: string | null;
}

const history = ref<HistoryEntry[]>([]);
const stats = ref<HistoryStats | null>(null);
const selectedFilter = ref<number>(24); // 默认24小时

const timeFilters = [
  { label: '24小时', value: 24 },
  { label: '7天', value: 168 },
  { label: '30天', value: 720 },
  { label: '全部', value: 0 },
];

const displayHistory = computed(() => history.value.slice(0, 50));

const chartPoints = computed(() => {
  if (history.value.length < 2) return '';
  const data = history.value.slice(0, 50).reverse();
  return data.map((item, index) => {
    const x = (index / (data.length - 1)) * 300;
    const y = 100 - (item.tokens_percent / 100) * 80 - 10;
    return `${x},${y}`;
  }).join(' ');
});

const chartPointsArray = computed(() => {
  if (history.value.length < 2) return [];
  const data = history.value.slice(0, 50).reverse();
  return data.map((item, index) => ({
    x: (index / (data.length - 1)) * 300,
    y: 100 - (item.tokens_percent / 100) * 80 - 10,
  }));
});

const chartColor = computed(() => {
  if (history.value.length === 0) return '#10B981';
  const latest = history.value[0];
  if (latest.tokens_percent <= 30) return '#10B981';
  if (latest.tokens_percent <= 60) return '#3B82F6';
  if (latest.tokens_percent <= 80) return '#F59E0B';
  return '#F97316';
});

const timeRangeLabel = computed(() => {
  const filter = timeFilters.find(f => f.value === selectedFilter.value);
  return filter?.label || '';
});

function getStatusClass(percent: number) {
  if (percent <= 30) return 'status-fresh';
  if (percent <= 60) return 'status-flow';
  if (percent <= 80) return 'status-warning';
  return 'status-panic';
}

function formatTime(timestamp: string) {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const hours = Math.floor(diff / (1000 * 60 * 60));
  
  if (hours < 1) return '刚刚';
  if (hours < 24) return `${hours}小时前`;
  const days = Math.floor(hours / 24);
  if (days < 7) return `${days}天前`;
  return date.toLocaleDateString('zh-CN');
}

async function selectFilter(hours: number) {
  selectedFilter.value = hours;
  await loadHistory();
}

async function loadHistory() {
  try {
    history.value = await invoke('get_history', { hours: selectedFilter.value });
    stats.value = await invoke('get_history_stats');
  } catch (e) {
    console.error('Failed to load history:', e);
  }
}

async function exportCsv() {
  try {
    const csv = await invoke('export_history_csv', { hours: selectedFilter.value });
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `usage-history-${new Date().toISOString().slice(0, 10)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  } catch (e) {
    console.error('Failed to export:', e);
  }
}

async function cleanup() {
  const days = prompt('保留最近多少天的数据？', '30');
  if (days && !isNaN(Number(days))) {
    try {
      const result = await invoke('cleanup_history', { days: Number(days) });
      alert(result);
      await loadHistory();
    } catch (e) {
      console.error('Failed to cleanup:', e);
    }
  }
}

onMounted(() => {
  loadHistory();
});
</script>

<style scoped>
.history-panel {
  padding: 16px;
  background: rgba(18, 20, 28, 0.95);
  border-radius: 12px;
  color: #e5e7eb;
  max-width: 400px;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.history-header h3 {
  margin: 0;
  font-size: 16px;
}

.time-filters {
  display: flex;
  gap: 4px;
}

.time-filters button {
  padding: 4px 8px;
  border: 1px solid #374151;
  background: transparent;
  color: #9ca3af;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.time-filters button.active {
  background: #3b82f6;
  border-color: #3b82f6;
  color: white;
}

.stats-summary {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 8px;
}

.stat-item {
  flex: 1;
  text-align: center;
}

.stat-label {
  display: block;
  font-size: 11px;
  color: #9ca3af;
  margin-bottom: 4px;
}

.stat-value {
  display: block;
  font-size: 16px;
  font-weight: bold;
  color: #10b981;
}

.trend-chart {
  margin-bottom: 16px;
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 8px;
}

.chart-svg {
  width: 100%;
  height: 80px;
}

.chart-labels {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  color: #6b7280;
  margin-top: 4px;
}

.history-list {
  max-height: 200px;
  overflow-y: auto;
  margin-bottom: 16px;
}

.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.history-time {
  font-size: 11px;
  color: #9ca3af;
  width: 60px;
}

.history-values {
  display: flex;
  gap: 12px;
}

.history-value {
  font-size: 12px;
}

.status-fresh { color: #10b981; }
.status-flow { color: #3b82f6; }
.status-warning { color: #f59e0b; }
.status-panic { color: #f97316; }

.no-data {
  text-align: center;
  padding: 20px;
  color: #6b7280;
}

.history-actions {
  display: flex;
  gap: 8px;
}

.history-actions button {
  flex: 1;
  padding: 8px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
}

.btn-export {
  background: #3b82f6;
  color: white;
}

.btn-cleanup {
  background: #374151;
  color: #e5e7eb;
}
</style>
