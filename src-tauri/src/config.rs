use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    #[serde(default)]
    pub display_config: DisplayConfig,
    #[serde(default)]
    pub pet_config: PetConfig,
    #[serde(default)]
    pub basic_config: BasicConfig,
    #[serde(default)]
    pub threshold_config: ThresholdConfig,
    #[serde(default)]
    pub growth_data: PetGrowthData,
    #[serde(default)]
    pub notification_config: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub display_mode: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            display_mode: "holo-bubble".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetConfig {
    pub selected_pet: String,
    pub action_interval: u64,
    #[serde(default)]
    pub accessories: PetAccessories,
}

/// 宠物配件配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetAccessories {
    /// 墨镜
    #[serde(default)]
    pub sunglasses: bool,
    /// 创口贴
    #[serde(default)]
    pub bandage: bool,
    /// 蝴蝶结
    #[serde(default)]
    pub bow: bool,
    /// 帽子
    #[serde(default)]
    pub hat: Option<String>,  // "cap", "beanie", "straw_hat"
}

impl Default for PetConfig {
    fn default() -> Self {
        Self {
            selected_pet: "cat".to_string(),
            action_interval: 25,
            accessories: PetAccessories::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicConfig {
    #[serde(default = "default_enable_glow")]
    pub enable_glow: bool,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

/// 颜色阈值配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// 各状态的百分比上限（不包含该值）
    /// 例如: { fresh: 25, flow: 50, warning: 75, panic: 90 }
    /// 表示: 0-25% Fresh, 26-50% Flow, 51-75% Warning, 76-90% Panic, 91%+ Exhausted
    #[serde(default = "default_fresh_threshold")]
    pub fresh_threshold: u32,
    #[serde(default = "default_flow_threshold")]
    pub flow_threshold: u32,
    #[serde(default = "default_warning_threshold")]
    pub warning_threshold: u32,
    #[serde(default = "default_panic_threshold")]
    pub panic_threshold: u32,
    /// 自定义颜色（可选，为空时使用默认颜色）
    #[serde(default)]
    pub fresh_color: Option<String>,
    #[serde(default)]
    pub flow_color: Option<String>,
    #[serde(default)]
    pub warning_color: Option<String>,
    #[serde(default)]
    pub panic_color: Option<String>,
    #[serde(default)]
    pub exhausted_color: Option<String>,
}

fn default_fresh_threshold() -> u32 { 25 }
fn default_flow_threshold() -> u32 { 50 }
fn default_warning_threshold() -> u32 { 75 }
fn default_panic_threshold() -> u32 { 90 }

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            fresh_threshold: 25,
            flow_threshold: 50,
            warning_threshold: 75,
            panic_threshold: 90,
            fresh_color: None,
            flow_color: None,
            warning_color: None,
            panic_color: None,
            exhausted_color: None,
        }
    }
}

/// 宠物成长数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetGrowthData {
    /// 当前等级 (1-10)
    #[serde(default)]
    pub level: u32,
    /// 当前经验值
    #[serde(default)]
    pub current_xp: u32,
    /// 累计总经验值
    #[serde(default)]
    pub total_xp: u32,
    /// 今日最高用完率
    #[serde(default)]
    pub today_max_percent: u32,
    /// 今日是否已领取奖励
    #[serde(default)]
    pub today_claimed: bool,
    /// 连续高用完率天数 (≥70%)
    #[serde(default)]
    pub high_usage_streak: u32,
    /// 今日日期 (YYYY-MM-DD)
    #[serde(default)]
    pub today_date: String,
    /// 已达成的用完率里程碑
    #[serde(default)]
    pub milestones_reached: Vec<u32>,
    /// 已解锁的物品
    #[serde(default)]
    pub unlocked_items: Vec<String>,
}

/// 等级阈值
const LEVEL_THRESHOLDS: [u32; 10] = [
    0,      // Lv.1
    500,    // Lv.2
    1500,   // Lv.3
    3000,   // Lv.4
    5000,   // Lv.5
    8000,   // Lv.6
    12000,  // Lv.7
    20000,  // Lv.8
    30000,  // Lv.9
    50000,  // Lv.10
];

impl PetGrowthData {
    /// 根据经验值计算等级
    pub fn calculate_level(xp: u32) -> u32 {
        for (i, &threshold) in LEVEL_THRESHOLDS.iter().enumerate().rev() {
            if xp >= threshold {
                return (i + 1) as u32;
            }
        }
        1
    }

    /// 添加经验值，返回是否升级
    pub fn add_xp(&mut self, xp: u32) -> bool {
        self.total_xp += xp;
        self.current_xp += xp;
        let new_level = Self::calculate_level(self.total_xp);
        let upgraded = new_level > self.level;
        if upgraded {
            self.level = new_level;
            self.check_unlocks();
        }
        upgraded
    }

    /// 根据用完率计算当日经验值
    pub fn calculate_daily_xp(percent: u32) -> u32 {
        let base_xp = match percent {
            p if p >= 95 => 100,
            p if p >= 85 => 80,
            p if p >= 70 => 50,
            p if p >= 50 => 30,
            _ => 10,
        };
        let mut xp = base_xp;
        // 完美用完奖励
        if percent >= 95 {
            xp += 50;
        }
        // 100% 用完奖励
        if percent >= 100 {
            xp += 30;
        }
        xp
    }

    /// 领取每日奖励
    pub fn claim_daily_reward(&mut self, current_percent: u32, date: String) -> Result<(u32, bool), String> {
        // 检查日期是否变更
        if self.today_date != date {
            // 新的一天，重置
            self.today_date = date;
            self.today_max_percent = current_percent;
            self.today_claimed = false;
        }

        // 更新今日最高用完率
        if current_percent > self.today_max_percent {
            self.today_max_percent = current_percent;
        }

        // 检查是否已领取
        if self.today_claimed {
            return Err("今日奖励已领取".to_string());
        }

        // 计算经验值
        let xp = Self::calculate_daily_xp(self.today_max_percent);

        // 检查里程碑
        let milestone_reached = self.check_milestones(self.today_max_percent);

        // 连续高用完率检查
        if self.today_max_percent >= 70 {
            self.high_usage_streak += 1;
            // 连续7天奖励
            if self.high_usage_streak >= 7 {
                self.high_usage_streak = 0; // 重置
                return Ok((xp + 100, true)); // 额外100 XP
            }
        } else {
            self.high_usage_streak = 0;
        }

        // 添加经验
        let upgraded = self.add_xp(xp);
        self.today_claimed = true;

        Ok((xp, upgraded))
    }

    /// 检查里程碑
    fn check_milestones(&mut self, percent: u32) -> bool {
        let milestones = [50, 70, 85, 95, 100];
        let mut reached = false;
        for &m in &milestones {
            if percent >= m && !self.milestones_reached.contains(&m) {
                self.milestones_reached.push(m);
                reached = true;
            }
        }
        reached
    }

    /// 检查并解锁物品
    fn check_unlocks(&mut self) {
        let level = self.level;
        let unlocks = [
            (2, vec![
                "trail-rainbow", "emoji-basic", "sound-basic",
            ]),
            (3, vec![
                "particles-stars", "aura-fire", "aura-frost", "effect-lightning",
            ]),
            (4, vec![
                "mask-temp", "broadcast-status", "color-presets", "particles-sakura",
            ]),
            (5, vec![
                "mode-vortex", "particles-diamond", "transform-butterfly", "mode-circus",
            ]),
            (6, vec![
                "mode-wave", "firework-levelup", "sound-full", "skin-night",
            ]),
            (7, vec![
                "mode-hologram", "pulse-energy", "pet-dye", "particles-legendary",
            ]),
            (8, vec![
                "crown-legendary", "mode-party", "transform-unicorn", "titles",
            ]),
            (9, vec![
                "pet-rainbow", "sound-custom", "custom-infinite", "mode-cosmic",
            ]),
            (10, vec![
                "aura-supreme", "mystery-box", "effects-all", "hall-of-fame",
            ]),
        ];

        for (lvl, items) in unlocks {
            if level >= lvl {
                for item in items {
                    let item_str = item.to_string();
                    if !self.unlocked_items.contains(&item_str) {
                        self.unlocked_items.push(item_str);
                    }
                }
            }
        }
    }

    /// 检查物品是否已解锁
    pub fn is_unlocked(&self, item: &str) -> bool {
        self.unlocked_items.contains(&item.to_string())
    }

    /// 获取下一级所需经验
    pub fn next_level_xp(&self) -> Option<u32> {
        if self.level >= 10 {
            return None;
        }
        LEVEL_THRESHOLDS.get(self.level as usize).copied()
    }

    /// 获取升级进度 (0.0 - 1.0)
    pub fn level_progress(&self) -> f64 {
        if self.level >= 10 {
            return 1.0;
        }
        let current_threshold = LEVEL_THRESHOLDS[(self.level - 1) as usize];
        let next_threshold = LEVEL_THRESHOLDS[self.level as usize];
        let progress = (self.total_xp - current_threshold) as f64
            / (next_threshold - current_threshold) as f64;
        progress.min(1.0)
    }
}

fn default_enable_glow() -> bool {
    true
}

fn default_theme() -> String {
    "dark".to_string()
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            enable_glow: true,
            auto_start: false,
            theme: "dark".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub current_model: String,
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub name: String,
    pub api_key: String,
    pub enabled: bool,
}

impl ModelConfig {
    /// 获取 API 域名（硬编码）
    pub fn api_domain(&self) -> &'static str {
        match self.provider.as_str() {
            "bigmodel" => "https://open.bigmodel.cn/",
            "zai" => "https://api.z.ai/",
            _ => "https://open.bigmodel.cn/",
        }
    }

    /// 获取完整的 API URL
    pub fn api_url(&self) -> String {
        format!("{}api/monitor/usage/quota/limit", self.api_domain())
    }
}

// 旧版配置结构（用于迁移）
#[derive(Debug, Clone, Deserialize)]
struct LegacyModelConfig {
    pub provider: String,
    pub name: String,
    pub api_url: String,
    pub api_key: String,
    pub enabled: bool,
}

impl From<LegacyModelConfig> for ModelConfig {
    fn from(legacy: LegacyModelConfig) -> Self {
        // 从完整 URL 中提取域名并丢弃（使用硬编码的域名）
        Self {
            provider: legacy.provider,
            name: legacy.name,
            api_key: legacy.api_key,
            enabled: legacy.enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollingConfig {
    pub interval_minutes: u64,
    /// 是否启用自适应轮询
    #[serde(default)]
    pub adaptive_polling: bool,
    /// 自适应轮询配置
    #[serde(default)]
    pub adaptive_config: AdaptivePollingConfig,
}

/// 自适应轮询配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdaptivePollingConfig {
    /// 高使用率时的轮询间隔（分钟）
    #[serde(default = "default_high_usage_interval")]
    pub high_usage_interval: u64,
    /// 低使用率时的轮询间隔（分钟）
    #[serde(default = "default_low_usage_interval")]
    pub low_usage_interval: u64,
    /// 高使用率阈值（%）
    #[serde(default = "default_high_usage_threshold")]
    pub high_usage_threshold: u32,
    /// 低使用率阈值（%）
    #[serde(default = "default_low_usage_threshold")]
    pub low_usage_threshold: u32,
    /// 配额耗尽时是否暂停轮询
    #[serde(default)]
    pub pause_when_exhausted: bool,
}

fn default_high_usage_interval() -> u64 { 1 }
fn default_low_usage_interval() -> u64 { 5 }
fn default_high_usage_threshold() -> u32 { 70 }
fn default_low_usage_threshold() -> u32 { 30 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_config: ApiConfig::default(),
            polling_config: PollingConfig::default(),
            display_config: DisplayConfig::default(),
            pet_config: PetConfig::default(),
            basic_config: BasicConfig::default(),
            threshold_config: ThresholdConfig::default(),
            growth_data: PetGrowthData::default(),
            notification_config: NotificationConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            current_model: "bigmodel".to_string(),
            models: vec![
                ModelConfig {
                    provider: "bigmodel".to_string(),
                    name: "BigModel".to_string(),
                    api_key: "".to_string(),
                    enabled: true,
                },
                ModelConfig {
                    provider: "zai".to_string(),
                    name: "Z.AI".to_string(),
                    api_key: "".to_string(),
                    enabled: true,
                },
            ],
        }
    }
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            interval_minutes: 1,
            adaptive_polling: false,
            adaptive_config: AdaptivePollingConfig::default(),
        }
    }
}

pub fn load_config(app: &AppHandle) -> Result<AppConfig, String> {
    let config_path = app.path().app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;

    if !config_path.exists() {
        fs::create_dir_all(&config_path)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }

    let config_file = config_path.join(CONFIG_FILE);

    if !config_file.exists() {
        let default_config = AppConfig::default();
        save_config(app, &default_config)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    // 尝试解析新格式
    match serde_json::from_str::<AppConfig>(&content) {
        Ok(config) => Ok(config),
        Err(_) => {
            // 新格式解析失败，尝试迁移旧格式
            #[derive(Deserialize)]
            struct LegacyAppConfig {
                pub api_config: LegacyApiConfig,
                pub polling_config: PollingConfig,
                pub display_config: Option<DisplayConfig>,
            }
            #[derive(Deserialize)]
            struct LegacyApiConfig {
                pub current_model: String,
                pub models: Vec<LegacyModelConfig>,
            }

            if let Ok(legacy_config) = serde_json::from_str::<LegacyAppConfig>(&content) {
                // 迁移旧配置到新格式
                let migrated_config = AppConfig {
                    api_config: ApiConfig {
                        current_model: legacy_config.api_config.current_model,
                        models: legacy_config.api_config.models
                            .into_iter()
                            .map(|m| ModelConfig::from(m))
                            .collect(),
                    },
                    polling_config: legacy_config.polling_config,
                    display_config: legacy_config.display_config.unwrap_or_default(),
                    pet_config: PetConfig::default(),
                    basic_config: BasicConfig::default(),
                    threshold_config: ThresholdConfig::default(),
                    growth_data: PetGrowthData::default(),
                    notification_config: NotificationConfig::default(),
                };
                // 保存迁移后的配置
                save_config(app, &migrated_config)?;
                Ok(migrated_config)
            } else {
                Err("Failed to parse config: invalid format".to_string())
            }
        }
    }
}

/// 预警通知配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationConfig {
    /// 是否启用通知
    #[serde(default)]
    pub enabled: bool,
    /// 预警阈值（超过此值发送通知）
    #[serde(default = "default_notification_threshold")]
    pub threshold: u32,
    /// 是否启用声音
    #[serde(default)]
    pub sound_enabled: bool,
    /// 通知冷却时间（分钟），避免频繁通知
    #[serde(default = "default_notification_cooldown")]
    pub cooldown_minutes: u32,
    /// 上次通知时间
    #[serde(default)]
    pub last_notification_time: Option<i64>,
}

fn default_notification_threshold() -> u32 { 80 }
fn default_notification_cooldown() -> u32 { 30 }

pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let config_path = app.path().app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;
    let config_file = config_path.join(CONFIG_FILE);

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_file, content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

pub fn get_active_model_config(app: &AppHandle) -> Result<ModelConfig, String> {
    let config = load_config(app)?;
    config.api_config.models
        .into_iter()
        .find(|m| m.provider == config.api_config.current_model)
        .ok_or_else(|| "Active model not found".to_string())
}
