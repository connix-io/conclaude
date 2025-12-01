use conclaude::database::{get_connection, get_database_path, HookExecution};
use sea_orm::EntityTrait;

#[tokio::test]
async fn test_database_initialization() {
    // Set a custom data dir for testing
    let test_dir = format!("/tmp/conclaude-test-db-{}", std::process::id());
    std::env::set_var("CONCLAUDE_DATA_DIR", &test_dir);

    // Get the database path
    let db_path = get_database_path().unwrap();
    assert!(db_path.to_string_lossy().contains("conclaude-test-db"));

    // Initialize connection (should run migrations)
    let conn = get_connection().await.unwrap();
    assert!(conn.ping().await.is_ok());

    // Verify table exists by querying it
    let result = HookExecution::find().all(conn).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);

    // Clean up
    std::fs::remove_dir_all(&test_dir).ok();
}
