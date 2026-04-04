use crate::events::{ApiResponse, UsageData, EVENT_USAGE_UPDATE};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

const API_URL: &str = "https://www.bigmodel.cn/api/monitor/usage/quota/limit";
// TODO: 生产环境应从配置文件或环境变量读取
const AUTH_TOKEN: &str = "eyJhbGciOiJIUzUxMiJ9.eyJ1c2VyX3R5cGUiOiJQRVJTT05BTCIsInVzZXJfY2hhbm5lbCI6IldFQiIsInVzZXJfaWQiOjE5Nzk5NTksInVzZXJfa2V5IjoiZjhiNjg2ZjUtNGQ0MS00ZjYxLTk0ODAtYjQzYTg4NzRjZmQ3IiwiY3VzdG9tZXJfaWQiOiIzNTg3MTc0ODkwNTE3NzAxOSIsInVzZXJuYW1lIjoiZWVpbXc5NjgifQ.V2C6pwp7plKnI1GlatOcEAcvlc3lxzD5IBeusQ4iOtxGpWsi2Y4eKlwUzW9YXF0sHBbMeqk3ZXn5GBjRS_UgKA";

/// 请求真实 API 获取用量数据
pub async fn fetch_usage() -> Result<UsageData, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get(API_URL)
        .header("authorization", AUTH_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let api_resp: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if !api_resp.success {
        return Err(format!("API returned error code: {}", api_resp.code));
    }

    let data = api_resp.data.ok_or("API response missing data field")?;

    let mut time_percent: u32 = 0;
    let mut time_remaining: Option<u32> = None;
    let mut tokens_percent: u32 = 0;

    for item in &data.limits {
        match item.limit_type.as_str() {
            "TIME_LIMIT" => {
                time_percent = item.percentage.unwrap_or(0);
                time_remaining = item.remaining;
            }
            "TOKENS_LIMIT" => {
                tokens_percent = item.percentage.unwrap_or(0);
            }
            _ => {}
        }
    }

    Ok(UsageData {
        // 宠物状态由 5h Token 额度（tokens_percent）驱动
        used: tokens_percent,
        total: 100,
        time_percent,
        tokens_percent,
        time_remaining,
    })
}

/// 启动轮询循环（每 5 分钟刷新一次）
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 启动时立即拉取一次，无需等待 5 分钟
    emit_usage(&app).await;

    let mut timer = interval(Duration::from_secs(300)); // 5 分钟
    timer.tick().await; // 跳过第一个立即触发的 tick

    loop {
        timer.tick().await;
        emit_usage(&app).await;
    }
}

async fn emit_usage(app: &AppHandle) {
    match fetch_usage().await {
        Ok(data) => {
            if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                eprintln!("Failed to emit usage update: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch usage: {}", e);
            let _ = app.emit("usage-error", format!("Usage fetch failed: {}", e));
        }
    }
}
