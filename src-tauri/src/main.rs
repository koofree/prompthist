// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest;

mod models;
mod prompt_storage;
mod crypto;
mod monitor;

use crate::models::*;
use crate::prompt_storage::PromptDatabase;
use crate::monitor::SystemMonitor;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn save_prompt(
    prompt: PromptEntry,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<String, String> {
    let mut entry = prompt;
    entry.starred = false;
    entry.tags = vec![];
    entry.usage_count = 0;
    entry.is_encrypted = false;

    match state.db.save_prompt(&entry).await {
        Ok(_) => Ok("Prompt saved successfully".to_string()),
        Err(e) => Err(format!("Failed to save prompt: {}", e)),
    }
}

#[tauri::command]
async fn get_prompts(
    filter: PromptFilter,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<PromptEntry>, String> {
    match state.db.get_prompts(Some(filter), None, None).await {
        Ok(prompts) => Ok(prompts),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_prompt_by_id(
    id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Option<PromptEntry>, String> {
    match state.db.get_prompt_by_id(&id).await {
        Ok(prompt) => Ok(prompt),
        Err(e) => Err(format!("Failed to get prompt: {}", e)),
    }
}

#[tauri::command]
async fn update_prompt(
    id: String,
    content: Option<String>,
    starred: Option<bool>,
    tags: Option<Vec<String>>,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<String, String> {
    match state.db.update_prompt(&id, content, starred, tags).await {
        Ok(_) => Ok("Prompt updated successfully".to_string()),
        Err(e) => Err(format!("Failed to update prompt: {}", e)),
    }
}

#[tauri::command]
async fn delete_prompt(
    id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<String, String> {
    match state.db.delete_prompt(&id).await {
        Ok(_) => Ok("Prompt deleted successfully".to_string()),
        Err(e) => Err(format!("Failed to delete prompt: {}", e)),
    }
}

#[tauri::command]
async fn get_prompt_stats(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<PromptStats, String> {
    match state.db.get_prompt_stats().await {
        Ok(stats) => Ok(stats),
        Err(e) => Err(format!("Failed to get stats: {}", e)),
    }
}

#[tauri::command]
async fn search_prompts(
    query: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<PromptEntry>, String> {
    match state.db.search_prompts(&query, None).await {
        Ok(prompts) => Ok(prompts),
        Err(e) => Err(format!("Failed to search prompts: {}", e)),
    }
}

#[tauri::command]
async fn get_system_info() -> std::result::Result<SystemInfo, String> {
    let system_info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_cores: num_cpus::get(),
        memory_total: 0, // Would need system crate for actual values
        memory_available: 0,
        disk_space: 0,
    };

    Ok(system_info)
}

#[tauri::command]
async fn start_monitoring(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<(), String> {
    let monitor_arc = Arc::clone(&state.monitor);
    let mut monitor = monitor_arc.lock().await;
    monitor.start_monitoring().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn stop_monitoring(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<(), String> {
    let monitor_arc = Arc::clone(&state.monitor);
    let mut monitor = monitor_arc.lock().await;
    monitor.stop_monitoring().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_monitoring_status(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<bool, String> {
    let monitor = state.monitor.lock().await;
    Ok(monitor.is_running())
}

#[tauri::command]
async fn send_prompt_to_ollama(
    request: OllamaRequest,
) -> std::result::Result<OllamaResponse, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let ollama_response: OllamaResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(ollama_response)
}

struct AppState {
    db: Arc<PromptDatabase>,
    monitor: Arc<Mutex<SystemMonitor>>,
}

#[tokio::main]
async fn main() {
    let db = match PromptDatabase::new().await {
        Ok(database) => Arc::new(database),
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    let config = MonitoringConfig::default();
    let monitor = Arc::new(Mutex::new(SystemMonitor::new(config, db.clone())));

    let app_state = AppState {
        db,
        monitor,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            save_prompt,
            get_prompts,
            get_prompt_by_id,
            update_prompt,
            delete_prompt,
            get_prompt_stats,
            search_prompts,
            get_system_info,
            start_monitoring,
            stop_monitoring,
            get_monitoring_status,
            send_prompt_to_ollama
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
