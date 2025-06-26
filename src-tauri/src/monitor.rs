use std::sync::{Arc, Mutex};
use std::process::Command;
use tokio::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use chrono::Utc;
use uuid::Uuid;
use crate::models::{MonitoringConfig, DetectedApplication, PromptHistError};
use crate::prompt_storage::PromptDatabase;
use std::collections::HashMap;

pub struct SystemMonitor {
    pub config: MonitoringConfig,
    db: Arc<PromptDatabase>,
    running: Arc<Mutex<bool>>,
    detected_apps: Arc<Mutex<Vec<DetectedApplication>>>,
    sender: Option<mpsc::UnboundedSender<String>>,
    recent_prompts: Arc<Mutex<HashMap<String, chrono::DateTime<Utc>>>>, // content -> timestamp
}

impl SystemMonitor {
    pub fn new(config: MonitoringConfig, db: Arc<PromptDatabase>) -> Self {
        Self {
            config,
            db,
            running: Arc::new(Mutex::new(false)),
            detected_apps: Arc::new(Mutex::new(Vec::new())),
            sender: None,
            recent_prompts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start_monitoring(&mut self) -> std::result::Result<(), PromptHistError> {
        if *self.running.lock().unwrap() {
            println!("[MONITOR] Monitoring already running, skipping start");
            return Ok(());
        }

        println!("[MONITOR] 🚀 Starting monitoring system...");
        println!("[MONITOR] Configuration: enabled={}, auto_save={}, threshold={}",
            self.config.enabled, self.config.auto_save, self.config.capture_threshold);
        println!("[MONITOR] Monitored applications: {:?}", self.config.monitored_applications);

        *self.running.lock().unwrap() = true;

        let (tx, mut rx) = mpsc::unbounded_channel();
        self.sender = Some(tx);

        let config = self.config.clone();
        let db = Arc::clone(&self.db);
        let running = Arc::clone(&self.running);
        let detected_apps = Arc::clone(&self.detected_apps);
        let recent_prompts = Arc::clone(&self.recent_prompts);

        // Start monitoring tasks
        tokio::spawn(async move {
            println!("[MONITOR] ⚡ Monitoring tasks started:");
            println!("[MONITOR]   - Web browser monitoring: every 2 seconds");
            println!("[MONITOR]   - Desktop app monitoring: every 3 seconds");
            println!("[MONITOR]   - Clipboard monitoring: every 1 second");

            let mut web_interval = time::interval(Duration::from_secs(2));
            let mut desktop_interval = time::interval(Duration::from_secs(3));
            let mut clipboard_interval = time::interval(Duration::from_secs(1));

            loop {
                if !*running.lock().unwrap() {
                    break;
                }

                tokio::select! {
                    _ = web_interval.tick() => {
                        if let Err(e) = Self::monitor_web_browsers(&config, &detected_apps).await {
                            eprintln!("Web monitoring error: {}", e);
                        }
                    }
                    _ = desktop_interval.tick() => {
                        if let Err(e) = Self::monitor_desktop_apps(&config, &detected_apps).await {
                            eprintln!("Desktop monitoring error: {}", e);
                        }
                    }
                    _ = clipboard_interval.tick() => {
                        if let Err(e) = Self::monitor_clipboard(&config, &db, &recent_prompts).await {
                            eprintln!("Clipboard monitoring error: {}", e);
                        }
                    }
                }
            }
        });

        // Process captured prompts
        tokio::spawn(async move {
            while let Some(prompt) = rx.recv().await {
                // Process and save prompt
                println!("Captured prompt: {}", prompt);
            }
        });

        Ok(())
    }

    pub async fn stop_monitoring(&mut self) -> std::result::Result<(), PromptHistError> {
        println!("[MONITOR] 🛑 Stopping monitoring system...");
        *self.running.lock().unwrap() = false;
        self.sender = None;
        println!("[MONITOR] ✅ Monitoring stopped successfully");
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    pub fn get_detected_applications(&self) -> Vec<DetectedApplication> {
        self.detected_apps.lock().unwrap().clone()
    }

    async fn monitor_web_browsers(
        config: &MonitoringConfig,
        detected_apps: &Arc<Mutex<Vec<DetectedApplication>>>,
    ) -> std::result::Result<(), PromptHistError> {
        if !config.enabled {
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            let browsers = vec!["Safari", "Google Chrome", "Firefox", "Microsoft Edge"];

            for browser in browsers {
                if let Ok(tabs) = Self::get_browser_tabs_macos(browser).await {
                    for (title, url) in tabs {
                        if let Some(app_name) = Self::identify_llm_application(&url) {
                            println!("[MONITOR] Detected LLM application: {} in {} - {}", app_name, browser, title);

                            if config.monitored_applications.contains(&app_name) {
                                let detected_app = DetectedApplication {
                                    name: app_name.clone(),
                                    process_name: browser.to_string(),
                                    window_title: title,
                                    is_active: true,
                                    last_activity: Utc::now(),
                                };

                                let mut apps = detected_apps.lock().unwrap();
                                if !apps.iter().any(|a| a.name == detected_app.name && a.window_title == detected_app.window_title) {
                                    println!("[MONITOR] Adding new detected application: {}", app_name);
                                    apps.push(detected_app);
                                } else {
                                    println!("[MONITOR] Application {} already tracked", app_name);
                                }
                            } else {
                                println!("[MONITOR] Application {} not in monitored list", app_name);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn monitor_desktop_apps(
        config: &MonitoringConfig,
        detected_apps: &Arc<Mutex<Vec<DetectedApplication>>>,
    ) -> std::result::Result<(), PromptHistError> {
        if !config.enabled {
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(apps) = Self::get_running_applications_macos().await {
                for (name, _pid) in apps {
                    if config.monitored_applications.contains(&name) {
                        let detected_app = DetectedApplication {
                            name: name.clone(),
                            process_name: name.clone(),
                            window_title: String::new(),
                            is_active: true,
                            last_activity: Utc::now(),
                        };

                        let mut detected = detected_apps.lock().unwrap();
                        if !detected.iter().any(|a| a.name == name) {
                            detected.push(detected_app);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn monitor_clipboard(
        config: &MonitoringConfig,
        db: &Arc<PromptDatabase>,
        recent_prompts: &Arc<Mutex<HashMap<String, chrono::DateTime<Utc>>>>,
    ) -> std::result::Result<(), PromptHistError> {
        if !config.enabled {
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(content) = Self::get_clipboard_content_macos().await {
                // Log clipboard content detection
                if !content.is_empty() {
                    println!("[MONITOR] Clipboard content detected: {} chars", content.len());
                }

                if Self::looks_like_prompt(&content) {
                    println!("[MONITOR] Content identified as prompt: {}",
                        if content.len() > 100 {
                            format!("{}...", &content[..100])
                        } else {
                            content.clone()
                        }
                    );

                    if content.len() >= config.capture_threshold as usize {
                        println!("[MONITOR] Prompt meets length threshold ({} >= {})",
                            content.len(), config.capture_threshold);

                        // Check for duplicates in recent prompts
                        let now = Utc::now();
                        let is_duplicate = {
                            let mut recent = recent_prompts.lock().unwrap();
                            
                            // Clean up old entries (older than 1 hour)
                            let one_hour_ago = now - chrono::Duration::hours(1);
                            recent.retain(|_, timestamp| *timestamp > one_hour_ago);
                            
                            // Check if this content was recently seen (within last 5 minutes)
                            let five_minutes_ago = now - chrono::Duration::minutes(5);
                            if let Some(last_seen) = recent.get(&content) {
                                if *last_seen > five_minutes_ago {
                                    true // This is a duplicate
                                } else {
                                    recent.insert(content.clone(), now);
                                    false // Not a recent duplicate
                                }
                            } else {
                                recent.insert(content.clone(), now);
                                false // First time seeing this content
                            }
                        };
                        
                        if is_duplicate {
                            println!("[MONITOR] ⚠️  Duplicate prompt detected, skipping save (content seen within last 5 minutes)");
                            return Ok(());
                        }

                        if config.auto_save {
                            let entry = crate::models::PromptEntry {
                                id: Uuid::new_v4().to_string(),
                                content: content.clone(),
                                application: "clipboard".to_string(),
                                timestamp: chrono::Utc::now(),
                                starred: false,
                                tags: vec![],
                                usage_count: 0,
                                is_encrypted: false,
                            };

                            println!("[MONITOR] Attempting to save prompt with ID: {}", entry.id);

                            match db.save_prompt(&entry).await {
                                Ok(_) => {
                                    println!("[MONITOR] ✅ Successfully saved prompt: ID={}, length={}, app={}",
                                        entry.id, content.len(), entry.application);
                                }
                                Err(e) => {
                                    eprintln!("[MONITOR] ❌ Failed to save clipboard prompt: {}", e);
                                }
                            }
                        } else {
                            println!("[MONITOR] Auto-save disabled, prompt not saved");
                        }
                    } else {
                        println!("[MONITOR] Prompt too short ({} < {}), skipping",
                            content.len(), config.capture_threshold);
                    }
                } else if !content.is_empty() {
                    println!("[MONITOR] Content not identified as prompt ({}...)",
                        if content.len() > 50 { &content[..50] } else { &content });
                }
            } else {
                // Only log clipboard read errors occasionally to avoid spam
                use std::sync::Mutex;
                use std::time::Instant;
                static LAST_CLIPBOARD_ERROR: Mutex<Option<Instant>> = Mutex::new(None);

                let now = Instant::now();
                let mut last_error = LAST_CLIPBOARD_ERROR.lock().unwrap();

                let should_log = match *last_error {
                    None => true,
                    Some(last) => now.duration_since(last).as_secs() > 60,
                };

                if should_log {
                    println!("[MONITOR] Failed to read clipboard content");
                    *last_error = Some(now);
                }
            }
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    async fn get_browser_tabs_macos(browser: &str) -> std::result::Result<Vec<(String, String)>, PromptHistError> {
        let script = match browser {
            "Safari" => r#"
                tell application "Safari"
                    set tabList to {}
                    repeat with w in windows
                        repeat with t in tabs of w
                            set end of tabList to (name of t) & "|" & (URL of t)
                        end repeat
                    end repeat
                    return tabList
                end tell
            "#,
            "Google Chrome" => r#"
                tell application "Google Chrome"
                    set tabList to {}
                    repeat with w in windows
                        repeat with t in tabs of w
                            set end of tabList to (title of t) & "|" & (URL of t)
                        end repeat
                    end repeat
                    return tabList
                end tell
            "#,
            _ => return Ok(vec![]),
        };

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| PromptHistError::SystemError(format!("Failed to execute AppleScript: {}", e)))?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let tabs: Vec<(String, String)> = result
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            Ok(tabs)
        } else {
            Ok(vec![])
        }
    }

    #[cfg(target_os = "macos")]
    async fn get_running_applications_macos() -> std::result::Result<Vec<(String, u32)>, PromptHistError> {
        let script = r#"
            tell application "System Events"
                set appList to {}
                repeat with p in application processes
                    if background only of p is false then
                        set end of appList to (name of p) & "|" & (unix id of p)
                    end if
                end repeat
                return appList
            end tell
        "#;

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| PromptHistError::SystemError(format!("Failed to execute AppleScript: {}", e)))?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            let apps: Vec<(String, u32)> = result
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() == 2 {
                        if let Ok(pid) = parts[1].parse::<u32>() {
                            Some((parts[0].to_string(), pid))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            Ok(apps)
        } else {
            Ok(vec![])
        }
    }

    #[cfg(target_os = "macos")]
    async fn get_clipboard_content_macos() -> std::result::Result<String, PromptHistError> {
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| PromptHistError::SystemError(format!("Failed to read clipboard: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(String::new())
        }
    }

    fn identify_llm_application(url: &str) -> Option<String> {
        let url_lower = url.to_lowercase();

        if url_lower.contains("chat.openai.com") || url_lower.contains("chatgpt") {
            Some("ChatGPT".to_string())
        } else if url_lower.contains("claude.ai") || url_lower.contains("anthropic") {
            Some("Claude".to_string())
        } else if url_lower.contains("cursor.sh") || url_lower.contains("cursor") {
            Some("Cursor".to_string())
        } else if url_lower.contains("x.ai") || url_lower.contains("grok") {
            Some("Grok".to_string())
        } else if url_lower.contains("perplexity.ai") {
            Some("Perplexity".to_string())
        } else if url_lower.contains("localhost:11434") || url_lower.contains("ollama") {
            Some("Ollama".to_string())
        } else {
            None
        }
    }

    fn looks_like_prompt(content: &str) -> bool {
        let content = content.trim();

        // Too short to be a meaningful prompt
        if content.len() < 10 {
            return false;
        }

        // Check for common prompt indicators
        let prompt_indicators = [
            "write", "create", "generate", "explain", "help", "how", "what", "why",
            "please", "can you", "could you", "would you", "i need", "i want",
            "make", "build", "design", "code", "program", "function", "class",
            "fix", "debug", "error", "issue", "problem", "solve", "analyze",
            "review", "improve", "optimize", "refactor", "translate", "convert",
            "summarize", "list", "compare", "difference", "pros and cons"
        ];

        let content_lower = content.to_lowercase();
        let word_count = content.split_whitespace().count();

        // Check if it looks like a question or command
        let has_question_mark = content.contains('?');
        let has_prompt_words = prompt_indicators.iter().any(|&indicator| content_lower.contains(indicator));
        let reasonable_length = word_count >= 3 && word_count <= 1000;

        // Additional checks for code-related prompts
        let has_code_keywords = content_lower.contains("function") ||
                               content_lower.contains("class") ||
                               content_lower.contains("variable") ||
                               content_lower.contains("algorithm") ||
                               content_lower.contains("syntax");

        reasonable_length && (has_question_mark || has_prompt_words || has_code_keywords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_llm_application() {
        assert_eq!(SystemMonitor::identify_llm_application("https://chat.openai.com/"), Some("ChatGPT".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("https://claude.ai/chat"), Some("Claude".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("https://cursor.sh/"), Some("Cursor".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("https://x.ai/grok"), Some("Grok".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("https://perplexity.ai/"), Some("Perplexity".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("http://localhost:11434/"), Some("Ollama".to_string()));
        assert_eq!(SystemMonitor::identify_llm_application("https://google.com/"), None);
    }

    #[test]
    fn test_looks_like_prompt() {
        assert!(SystemMonitor::looks_like_prompt("How do I create a function in Python?"));
        assert!(SystemMonitor::looks_like_prompt("Write a function that calculates fibonacci numbers"));
        assert!(SystemMonitor::looks_like_prompt("Can you help me debug this code?"));
        assert!(SystemMonitor::looks_like_prompt("Explain the difference between let and const"));
        assert!(!SystemMonitor::looks_like_prompt("hello"));
        assert!(!SystemMonitor::looks_like_prompt(""));
        assert!(!SystemMonitor::looks_like_prompt("a"));
    }
}
