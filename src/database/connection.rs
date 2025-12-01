use anyhow::{Context, Result};
use sea_orm::{Database, DatabaseConnection};
use std::path::PathBuf;
use std::sync::OnceLock;

use super::migrations::{Migrator, MigratorTrait};

/// Global database connection singleton
static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

/// Get the platform-specific data directory for conclaude
///
/// Priority:
/// 1. CONCLAUDE_DATA_DIR environment variable
/// 2. Platform-specific default:
///    - Linux: $XDG_DATA_HOME/conclaude or ~/.local/share/conclaude
///    - macOS: ~/Library/Application Support/conclaude
///    - Windows: %LOCALAPPDATA%\conclaude
pub fn get_data_dir() -> Result<PathBuf> {
    // Check for environment variable override
    if let Ok(custom_dir) = std::env::var("CONCLAUDE_DATA_DIR") {
        return Ok(PathBuf::from(custom_dir));
    }

    // Use platform-specific defaults via dirs crate
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
            return Ok(PathBuf::from(xdg_data).join("conclaude"));
        }
        if let Some(home) = dirs::home_dir() {
            return Ok(home.join(".local").join("share").join("conclaude"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            return Ok(home
                .join("Library")
                .join("Application Support")
                .join("conclaude"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(local_data) = dirs::data_local_dir() {
            return Ok(local_data.join("conclaude"));
        }
    }

    // Fallback for other platforms
    if let Some(home) = dirs::home_dir() {
        return Ok(home.join(".conclaude"));
    }

    anyhow::bail!("Unable to determine data directory for conclaude")
}

/// Get the full path to the database file
pub fn get_database_path() -> Result<PathBuf> {
    Ok(get_data_dir()?.join("conclaude.db"))
}

/// Get or initialize the database connection
///
/// This function uses a singleton pattern to ensure only one connection exists.
/// On first call, it will:
/// 1. Create the data directory if it doesn't exist
/// 2. Connect to the SQLite database (creating it if needed)
/// 3. Run all pending migrations
/// 4. Store the connection in a global singleton
///
/// Subsequent calls return the existing connection.
pub async fn get_connection() -> Result<&'static DatabaseConnection> {
    // If connection already exists, return it
    if let Some(conn) = DB_CONNECTION.get() {
        return Ok(conn);
    }

    // Create data directory if it doesn't exist
    let db_path = get_database_path()?;
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create data directory: {}", parent.display()))?;
    }

    // Connect to database (creates file if it doesn't exist)
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let db = Database::connect(&db_url)
        .await
        .with_context(|| format!("Failed to connect to database at {}", db_path.display()))?;

    // Run migrations
    Migrator::up(&db, None)
        .await
        .context("Failed to run database migrations")?;

    // Store in singleton (this will only succeed once)
    DB_CONNECTION
        .set(db)
        .map_err(|_| anyhow::anyhow!("Database connection already initialized"))?;

    // Return the stored connection
    DB_CONNECTION
        .get()
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve database connection"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data_dir_with_env() {
        let test_dir = "/tmp/conclaude-test";
        std::env::set_var("CONCLAUDE_DATA_DIR", test_dir);
        let result = get_data_dir().unwrap();
        assert_eq!(result, PathBuf::from(test_dir));
        std::env::remove_var("CONCLAUDE_DATA_DIR");
    }

    #[test]
    fn test_get_data_dir_default() {
        std::env::remove_var("CONCLAUDE_DATA_DIR");
        let result = get_data_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("conclaude"));
    }

    #[test]
    fn test_get_database_path() {
        let result = get_database_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("conclaude.db"));
    }
}
