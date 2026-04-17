/// 历史数据数据库模块
/// 使用 SQLite 存储使用量历史记录

use rusqlite::{Connection, Result, params};
use std::path::{PathBuf};
use dirs::config_dir;
use chrono::{Utc, DateTime};

/// 数据库文件名
const DB_NAME: &str = "plan-guard.db";

/// 获取数据库文件路径
pub fn get_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_dir = config_dir().ok_or("Failed to get config directory")?;
    config_dir.push("plan-guard");
    std::fs::create_dir_all(&config_dir)?;
    config_dir.push(DB_NAME);
    Ok(config_dir)
}

/// 初始化数据库，创建表结构
pub fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS usage_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            tokens_percent INTEGER NOT NULL,
            tokens_used INTEGER,
            tokens_total INTEGER DEFAULT 100,
            time_percent INTEGER NOT NULL,
            time_remaining INTEGER,
            weekly_tokens_percent INTEGER NOT NULL DEFAULT 0,
            weekly_tokens_used INTEGER,
            level TEXT
        )",
        []
    )?;

    // 创建索引以加速查询
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON usage_log(timestamp)",
        []
    )?;

    Ok(())
}

/// 记录使用量数据
#[derive(Debug, Clone)]
pub struct UsageLogEntry {
    pub tokens_percent: u32,
    pub tokens_used: Option<u64>,
    pub tokens_total: u32,
    pub time_percent: u32,
    pub time_remaining: Option<u32>,
    pub weekly_tokens_percent: u32,
    pub weekly_tokens_used: Option<u64>,
    pub level: Option<String>,
}

pub fn insert_usage_log(entry: &UsageLogEntry) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "INSERT INTO usage_log (
            tokens_percent, tokens_used, tokens_total,
            time_percent, time_remaining,
            weekly_tokens_percent, weekly_tokens_used, level
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            entry.tokens_percent,
            entry.tokens_used,
            entry.tokens_total,
            entry.time_percent,
            entry.time_remaining,
            entry.weekly_tokens_percent,
            entry.weekly_tokens_used,
            entry.level.as_deref(),
        ],
    )?;

    Ok(())
}

/// 历史数据查询结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub timestamp: String,
    pub tokens_percent: u32,
    pub tokens_used: Option<i64>,
    pub time_percent: u32,
    pub time_remaining: Option<i32>,
    pub weekly_tokens_percent: i32,
    pub level: Option<String>,
}

/// 查询指定时间范围内的历史数据
/// hours: 查询最近 N 小时的数据，0 表示全部
pub fn query_history(hours: u32) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    let query = if hours > 0 {
        "SELECT id, timestamp, tokens_percent, tokens_used, time_percent,
                time_remaining, weekly_tokens_percent, level
         FROM usage_log
         WHERE datetime(timestamp) >= datetime('now', '-' || ?1 || ' hours')
         ORDER BY timestamp DESC"
    } else {
        "SELECT id, timestamp, tokens_percent, tokens_used, time_percent,
                time_remaining, weekly_tokens_percent, level
         FROM usage_log
         ORDER BY timestamp DESC"
    };

    let mut stmt = if hours > 0 {
        conn.prepare(query)?
    } else {
        conn.prepare(query)?
    };

    let rows_iter = if hours > 0 {
        stmt.query_map([hours], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                tokens_percent: row.get(2)?,
                tokens_used: row.get(3)?,
                time_percent: row.get(4)?,
                time_remaining: row.get(5)?,
                weekly_tokens_percent: row.get(6)?,
                level: row.get(7)?,
            })
        })?
    } else {
        stmt.query_map([], |row| {
            Ok(HistoryEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                tokens_percent: row.get(2)?,
                tokens_used: row.get(3)?,
                time_percent: row.get(4)?,
                time_remaining: row.get(5)?,
                weekly_tokens_percent: row.get(6)?,
                level: row.get(7)?,
            })
        })?
    };

    let mut results = Vec::new();
    for row in rows_iter {
        results.push(row?);
    }

    Ok(results)
}

/// 获取统计摘要
#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoryStats {
    pub total_records: i64,
    pub avg_tokens_percent: f64,
    pub max_tokens_percent: u32,
    pub first_record: Option<String>,
    pub last_record: Option<String>,
}

pub fn get_history_stats() -> Result<HistoryStats, Box<dyn std::error::Error>> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    // 总记录数
    let total_records: i64 = conn.query_row(
        "SELECT COUNT(*) FROM usage_log",
        [],
        |row| row.get(0)
    )?;

    // 平均 Token 使用百分比
    let avg_tokens_percent: f64 = conn.query_row(
        "SELECT AVG(tokens_percent) FROM usage_log",
        [],
        |row| row.get(0)
    ).unwrap_or(0.0);

    // 最大 Token 使用百分比
    let max_tokens_percent: u32 = conn.query_row(
        "SELECT MAX(tokens_percent) FROM usage_log",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    // 最早和最新记录时间
    let first_record: Option<String> = conn.query_row(
        "SELECT MIN(timestamp) FROM usage_log",
        [],
        |row| row.get(0)
    )?;

    let last_record: Option<String> = conn.query_row(
        "SELECT MAX(timestamp) FROM usage_log",
        [],
        |row| row.get(0)
    )?;

    Ok(HistoryStats {
        total_records,
        avg_tokens_percent,
        max_tokens_percent,
        first_record,
        last_record,
    })
}

/// 删除旧数据（保留最近 N 天）
pub fn cleanup_old_data(days: u32) -> Result<usize, Box<dyn std::error::Error>> {
    let db_path = get_db_path()?;
    let conn = Connection::open(&db_path)?;

    let deleted = conn.execute(
        "DELETE FROM usage_log WHERE datetime(timestamp) < datetime('now', '-' || ?1 || ' days')",
        params![days]
    )?;

    Ok(deleted)
}

/// 导出历史数据为 CSV 格式
pub fn export_to_csv(hours: u32) -> Result<String, Box<dyn std::error::Error>> {
    let entries = query_history(hours)?;

    let mut csv = String::from("ID,Timestamp,Tokens%,TokensUsed,Time%,TimeRemaining,WeeklyTokens%,Level\n");

    for entry in entries {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            entry.id,
            entry.timestamp,
            entry.tokens_percent,
            entry.tokens_used.map_or("".to_string(), |v| v.to_string()),
            entry.time_percent,
            entry.time_remaining.map_or("".to_string(), |v| v.to_string()),
            entry.weekly_tokens_percent,
            entry.level.unwrap_or("".to_string())
        ));
    }

    Ok(csv)
}
