<template>
  <div class="notification-settings">
    <h3>🔔 预警通知</h3>

    <div class="setting-row">
      <label class="setting-label">
        <input
          type="checkbox"
          v-model="config.enabled"
          @change="saveConfig"
        />
        <span>启用预警通知</span>
      </label>
      <span class="setting-desc">当使用率超过阈值时发送通知</span>
    </div>

    <div class="setting-row" v-if="config.enabled">
      <label class="setting-label">
        预警阈值
        <input
          type="range"
          v-model.number="config.threshold"
          min="50"
          max="95"
          step="5"
          @input="saveConfig"
        />
        <span class="threshold-value">{{ config.threshold }}%</span>
      </label>
    </div>

    <div class="setting-row" v-if="config.enabled">
      <label class="setting-label">
        <input
          type="checkbox"
          v-model="config.sound_enabled"
          @change="saveConfig"
        />
        <span>启用通知声音</span>
      </label>
    </div>

    <div class="setting-row" v-if="config.enabled">
      <label class="setting-label">
        通知冷却时间
        <select v-model.number="config.cooldown_minutes" @change="saveConfig">
          <option :value="15">15 分钟</option>
          <option :value="30">30 分钟</option>
          <option :value="60">1 小时</option>
          <option :value="120">2 小时</option>
        </select>
      </label>
      <span class="setting-desc">避免频繁通知</span>
    </div>

    <div class="setting-row">
      <button @click="testNotification" class="btn-test">
        🔔 测试通知
      </button>
    </div>

    <!-- 预警状态预览 -->
    <div class="preview-section">
      <div class="preview-header">当前预警设置</div>
      <div class="preview-content">
        <div v-if="!config.enabled" class="preview-disabled">
          通知已禁用
        </div>
        <div v-else class="preview-enabled">
          当 Token 使用率超过
          <span class="highlight">{{ config.threshold }}%</span>
          时发送通知
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const config = ref({
  enabled: false,
  threshold: 80,
  sound_enabled: false,
  cooldown_minutes: 30,
});

async function loadConfig() {
  try {
    const appConfig = await invoke('get_config') as any;
    if (appConfig.notification_config) {
      config.value = {
        enabled: appConfig.notification_config.enabled || false,
        threshold: appConfig.notification_config.threshold || 80,
        sound_enabled: appConfig.notification_config.sound_enabled || false,
        cooldown_minutes: appConfig.notification_config.cooldown_minutes || 30,
      };
    }
  } catch (e) {
    console.error('Failed to load notification config:', e);
  }
}

async function saveConfig() {
  try {
    await invoke('update_notification_config', {
      enabled: config.value.enabled,
      threshold: config.value.threshold,
      soundEnabled: config.value.sound_enabled,
      cooldownMinutes: config.value.cooldown_minutes,
    });
  } catch (e) {
    console.error('Failed to save notification config:', e);
  }
}

async function testNotification() {
  try {
    await invoke('test_notification');
    const notification = new Notification('🔔 测试通知', {
      body: '这是一条测试通知',
      requireInteraction: false,
    });

    if (config.value.sound_enabled) {
      const audio = new Audio('data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+PwtmMcBjiR1/LMeSwFJHfH8N2QQAoUXrTp66hVFApGn+DyvmwhBSuBzvLZiTYIG2i98+/fTgwOU6jl77ZlHAY4k9fyyXksBShl1/DdkUAKFFmz6+qYRQJRp7g8r1vIQUqg8/y2Ik2CBtosvPw304MDlOg5d+2ZRwGOJPX8sl5LAUoZtfw3JFAChRUc/vqmEUCWaw/K9byEFKsPM9tiJNgsbcLDz9NMDA5S6Gl37ZlnAY4k9fyy3ksBShl1/DckUAImF');
      audio.volume = 0.3;
      audio.play().catch(() => {});
    }
  } catch (e) {
    console.error('Failed to send test notification:', e);
    alert('通知发送失败，请检查系统通知权限');
  }
}

onMounted(() => {
  loadConfig();

  if ('Notification' in window && Notification.permission === 'default') {
    Notification.requestPermission();
  }
});
</script>

<style scoped>
.notification-settings { padding: 16px; }
h3 { margin: 0 0 16px 0; font-size: 16px; color: #e5e7eb; }
.setting-row { display: flex; justify-content: space-between; align-items: center; padding: 12px 0; border-bottom: 1px solid rgba(255,255,255,0.1); }
.setting-row:last-child { border-bottom: none; }
.setting-label { display: flex; align-items: center; gap: 8px; color: #e5e7eb; font-size: 14px; }
.setting-label input[type="checkbox"] { width: 16px; height: 16px; }
.setting-label input[type="range"] { width: 120px; margin: 0 8px; }
.setting-label select { padding: 4px 8px; background: #1f2937; border: 1px solid #374151; color: #e5e7eb; border-radius: 4px; }
.threshold-value { min-width: 40px; text-align: center; font-weight: bold; color: #10b981; }
.setting-desc { font-size: 12px; color: #9ca3af; }
.btn-test { padding: 8px 16px; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer; }
.btn-test:hover { background: #2563eb; }
.preview-section { margin-top: 16px; padding: 12px; background: rgba(0,0,0,0.3); border-radius: 8px; }
.preview-header { font-size: 12px; color: #9ca3af; margin-bottom: 8px; }
.preview-disabled { color: #6b7280; }
.preview-enabled { color: #e5e7eb; }
.highlight { color: #f59e0b; font-weight: bold; }
</style>
