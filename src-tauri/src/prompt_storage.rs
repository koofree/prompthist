use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool, Row};
use dirs::data_local_dir;

use crate::models::{PromptEntry, PromptFilter, PromptStats, Result, PromptHistError};

pub struct PromptDatabase {
    pool: SqlitePool,
}

impl PromptDatabase {
    pub async fn new() -> Result<Self> {
        let db_path = Self::get_database_path()?;

        // Ensure the directory exists
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        let db = Self { pool };
        db.initialize_schema().await?;

        Ok(db)
    }

    fn get_database_path() -> Result<PathBuf> {
        let mut path = data_local_dir()
            .ok_or_else(|| PromptHistError::InvalidInput("Could not find data directory".to_string()))?;

        path.push("prompthist");
        path.push("prompts.db");

        Ok(path)
    }

    async fn initialize_schema(&self) -> Result<()> {
        // Create prompts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prompts (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                application TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                starred BOOLEAN NOT NULL DEFAULT FALSE,
                tags TEXT NOT NULL DEFAULT '[]',
                usage_count INTEGER NOT NULL DEFAULT 1,
                is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for better performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_prompts_application ON prompts(application)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_prompts_timestamp ON prompts(timestamp)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_prompts_starred ON prompts(starred)")
            .execute(&self.pool)
            .await?;

        // Create FTS table for full-text search
        sqlx::query(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS prompts_fts USING fts5(
                id,
                content,
                application,
                tags,
                content='prompts',
                content_rowid='rowid'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create triggers to keep FTS table in sync
        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS prompts_ai AFTER INSERT ON prompts BEGIN
                INSERT INTO prompts_fts(rowid, id, content, application, tags)
                VALUES (new.rowid, new.id, new.content, new.application, new.tags);
            END
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS prompts_ad AFTER DELETE ON prompts BEGIN
                INSERT INTO prompts_fts(prompts_fts, rowid, id, content, application, tags)
                VALUES('delete', old.rowid, old.id, old.content, old.application, old.tags);
            END
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS prompts_au AFTER UPDATE ON prompts BEGIN
                INSERT INTO prompts_fts(prompts_fts, rowid, id, content, application, tags)
                VALUES('delete', old.rowid, old.id, old.content, old.application, old.tags);
                INSERT INTO prompts_fts(rowid, id, content, application, tags)
                VALUES (new.rowid, new.id, new.content, new.application, new.tags);
            END
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_prompt(&self, prompt: &PromptEntry) -> Result<()> {
        let tags_json = serde_json::to_string(&prompt.tags)?;

        sqlx::query(
            r#"
            INSERT INTO prompts (id, content, application, timestamp, starred, tags, usage_count, is_encrypted)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&prompt.id)
        .bind(&prompt.content)
        .bind(&prompt.application)
        .bind(prompt.timestamp.to_rfc3339())
        .bind(if prompt.starred { 1 } else { 0 })
        .bind(tags_json)
        .bind(prompt.usage_count)
        .bind(if prompt.is_encrypted { 1 } else { 0 })
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_prompts(
        &self,
        filter: Option<PromptFilter>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<PromptEntry>> {
        let mut query = "SELECT * FROM prompts WHERE 1=1".to_string();
        let mut params: Vec<String> = Vec::new();

        if let Some(f) = filter {
            if let Some(app) = f.application {
                query.push_str(" AND application = ?");
                params.push(app);
            }
            if let Some(starred) = f.starred {
                query.push_str(" AND starred = ?");
                params.push(starred.to_string());
            }
            if let Some(start_date) = f.start_date {
                query.push_str(" AND timestamp >= ?");
                params.push(start_date.to_rfc3339());
            }
            if let Some(end_date) = f.end_date {
                query.push_str(" AND timestamp <= ?");
                params.push(end_date.to_rfc3339());
            }
        }

        query.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = sql_query.bind(param);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        let mut prompts = Vec::new();

        for row in rows {
            let tags_json: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| PromptHistError::InvalidInput(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc);

            prompts.push(PromptEntry {
                id: row.get("id"),
                content: row.get("content"),
                application: row.get("application"),
                timestamp,
                starred: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(starred_int) = row.try_get::<i32, _>("starred") {
                        starred_int != 0
                    } else if let Ok(starred_str) = row.try_get::<String, _>("starred") {
                        starred_str == "1" || starred_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
                tags,
                usage_count: row.get("usage_count"),
                is_encrypted: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(encrypted_int) = row.try_get::<i32, _>("is_encrypted") {
                        encrypted_int != 0
                    } else if let Ok(encrypted_str) = row.try_get::<String, _>("is_encrypted") {
                        encrypted_str == "1" || encrypted_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
            });
        }

        Ok(prompts)
    }

    pub async fn get_prompt_by_id(&self, id: &str) -> Result<Option<PromptEntry>> {
        let row = sqlx::query("SELECT * FROM prompts WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let tags_json: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| PromptHistError::InvalidInput(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc);

            Ok(Some(PromptEntry {
                id: row.get("id"),
                content: row.get("content"),
                application: row.get("application"),
                timestamp,
                starred: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(starred_int) = row.try_get::<i32, _>("starred") {
                        starred_int != 0
                    } else if let Ok(starred_str) = row.try_get::<String, _>("starred") {
                        starred_str == "1" || starred_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
                tags,
                usage_count: row.get("usage_count"),
                is_encrypted: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(encrypted_int) = row.try_get::<i32, _>("is_encrypted") {
                        encrypted_int != 0
                    } else if let Ok(encrypted_str) = row.try_get::<String, _>("is_encrypted") {
                        encrypted_str == "1" || encrypted_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_prompt(
        &self,
        id: &str,
        content: Option<String>,
        starred: Option<bool>,
        tags: Option<Vec<String>>,
    ) -> Result<()> {
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(content) = content {
            updates.push("content = ?");
            params.push(content);
        }
        if let Some(starred) = starred {
            updates.push("starred = ?");
            params.push(if starred { "1".to_string() } else { "0".to_string() });
        }
        if let Some(tags) = tags {
            updates.push("tags = ?");
            params.push(serde_json::to_string(&tags)?);
        }

        if updates.is_empty() {
            return Ok(());
        }

        updates.push("updated_at = CURRENT_TIMESTAMP");
        params.push(id.to_string());

        let query = format!(
            "UPDATE prompts SET {} WHERE id = ?",
            updates.join(", ")
        );

        let mut sql_query = sqlx::query(&query);
        for param in params {
            sql_query = sql_query.bind(param);
        }

        sql_query.execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_prompt(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM prompts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn search_prompts(&self, query: &str, limit: Option<i32>) -> Result<Vec<PromptEntry>> {
        let limit = limit.unwrap_or(50);

        let sql = r#"
            SELECT p.* FROM prompts p
            JOIN prompts_fts fts ON p.rowid = fts.rowid
            WHERE prompts_fts MATCH ?
            ORDER BY rank
            LIMIT ?
        "#;

        let rows = sqlx::query(sql)
            .bind(query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut prompts = Vec::new();
        for row in rows {
            let tags_json: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| PromptHistError::InvalidInput(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc);

            prompts.push(PromptEntry {
                id: row.get("id"),
                content: row.get("content"),
                application: row.get("application"),
                timestamp,
                starred: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(starred_int) = row.try_get::<i32, _>("starred") {
                        starred_int != 0
                    } else if let Ok(starred_str) = row.try_get::<String, _>("starred") {
                        starred_str == "1" || starred_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
                tags,
                usage_count: row.get("usage_count"),
                is_encrypted: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(encrypted_int) = row.try_get::<i32, _>("is_encrypted") {
                        encrypted_int != 0
                    } else if let Ok(encrypted_str) = row.try_get::<String, _>("is_encrypted") {
                        encrypted_str == "1" || encrypted_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
            });
        }

        Ok(prompts)
    }

    pub async fn get_prompt_stats(&self) -> Result<PromptStats> {
        // Get total prompts count
        let total_prompts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prompts")
            .fetch_one(&self.pool)
            .await?;

        // Get starred count
        let starred_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM prompts WHERE starred = true")
            .fetch_one(&self.pool)
            .await?;

        // Get applications breakdown
        let app_rows = sqlx::query("SELECT application, COUNT(*) as count FROM prompts GROUP BY application")
            .fetch_all(&self.pool)
            .await?;

        let mut applications = HashMap::new();
        for row in app_rows {
            let app: String = row.get("application");
            let count: i64 = row.get("count");
            applications.insert(app, count);
        }

        // Get most used prompts
        let most_used_rows = sqlx::query("SELECT * FROM prompts ORDER BY usage_count DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;

        let mut most_used_prompts = Vec::new();
        for row in most_used_rows {
            let tags_json: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| PromptHistError::InvalidInput(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc);

            most_used_prompts.push(PromptEntry {
                id: row.get("id"),
                content: row.get("content"),
                application: row.get("application"),
                timestamp,
                starred: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(starred_int) = row.try_get::<i32, _>("starred") {
                        starred_int != 0
                    } else if let Ok(starred_str) = row.try_get::<String, _>("starred") {
                        starred_str == "1" || starred_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
                tags,
                usage_count: row.get("usage_count"),
                is_encrypted: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(encrypted_int) = row.try_get::<i32, _>("is_encrypted") {
                        encrypted_int != 0
                    } else if let Ok(encrypted_str) = row.try_get::<String, _>("is_encrypted") {
                        encrypted_str == "1" || encrypted_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
            });
        }

        // Get recent activity
        let recent_rows = sqlx::query("SELECT * FROM prompts ORDER BY timestamp DESC LIMIT 10")
            .fetch_all(&self.pool)
            .await?;

        let mut recent_activity = Vec::new();
        for row in recent_rows {
            let tags_json: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| PromptHistError::InvalidInput(format!("Invalid timestamp: {}", e)))?
                .with_timezone(&Utc);

            recent_activity.push(PromptEntry {
                id: row.get("id"),
                content: row.get("content"),
                application: row.get("application"),
                timestamp,
                starred: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(starred_int) = row.try_get::<i32, _>("starred") {
                        starred_int != 0
                    } else if let Ok(starred_str) = row.try_get::<String, _>("starred") {
                        starred_str == "1" || starred_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
                tags,
                usage_count: row.get("usage_count"),
                is_encrypted: {
                    // Try to get as integer first, then fall back to string
                    if let Ok(encrypted_int) = row.try_get::<i32, _>("is_encrypted") {
                        encrypted_int != 0
                    } else if let Ok(encrypted_str) = row.try_get::<String, _>("is_encrypted") {
                        encrypted_str == "1" || encrypted_str.to_lowercase() == "true"
                    } else {
                        false
                    }
                },
            });
        }

        Ok(PromptStats {
            total_prompts,
            applications,
            starred_count,
            most_used_prompts,
            recent_activity,
        })
    }

    pub async fn increment_usage_count(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE prompts SET usage_count = usage_count + 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
