export type DisplayMode =
  | 'none'             // 无 - 不显示用量 (None)
  | 'holo-bubble'      // 全息像素气泡 (Holo-Bubble)
  | 'cyber-ring'       // 赛博能量环 (Cyber-Ring)
  | 'aura-field'       // 灵能力场波纹 (Aura-Field)
  | 'energy-core'      // 格子能量核心 (Energy-Core)
  | 'status-floater'   // 悬浮状态标 (Status-Floater)

export interface DisplayConfig {
  display_mode: DisplayMode
}

export interface ApiConfig {
  current_model: string
  models: ModelConfig[]
}

export interface ModelConfig {
  provider: string
  name: string
  enabled: boolean
  api_key: string
}

export interface PollingConfig {
  interval_minutes: number
}

// 宠物类型
export type PetType = 'spirit' | 'ghost'

// 猫咪动作（已废弃，保留以兼容）
export type CatAction = 'cat-sleep' | 'cat-play' | 'cat-stare' | 'cat-stretch'

// 狗狗动作（已废弃，保留以兼容）
export type DogAction = 'dog-sit' | 'dog-bark' | 'dog-walk' | 'dog-beg'

// 宠物配置
export interface PetConfig {
  selected_pet: PetType
  action_interval: number  // 秒
  accessories?: PetAccessories
}

// 宠物配件
export interface PetAccessories {
  sunglasses?: boolean   // 墨镜
  bandage?: boolean      // 创口贴
  bow?: boolean          // 蝴蝶结
  hat?: 'cap' | 'beanie' | 'straw_hat' | null  // 帽子
}

// 基础设置配置
export interface BasicConfig {
  enable_glow: boolean     // 光晕层开关
  auto_start: boolean      // 开机自启动
  theme: 'dark' | 'light'  // 主题模式
}

// 阈值配置
export interface ThresholdConfig {
  fresh_threshold: number       // Fresh 状态上限
  flow_threshold: number        // Flow 状态上限
  warning_threshold: number     // Warning 状态上限
  panic_threshold: number       // Panic 状态上限
  fresh_color?: string          // 自定义 Fresh 颜色
  flow_color?: string           // 自定义 Flow 颜色
  warning_color?: string        // 自定义 Warning 颜色
  panic_color?: string          // 自定义 Panic 颜色
  exhausted_color?: string      // 自定义 Exhausted 颜色
}

export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
  display_config: DisplayConfig
  pet_config: PetConfig
  basic_config: BasicConfig
  threshold_config?: ThresholdConfig
  growth_data?: PetGrowthData
}

// 宠物成长数据
export interface PetGrowthData {
  level: number           // 当前等级 (1-10)
  total_tokens: number    // 累计使用的 token 总数
  unlocked_items: string[] // 已解锁的物品
}
