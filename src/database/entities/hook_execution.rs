use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Hook execution record entity
///
/// Stores information about each hook execution including timing,
/// status, and context information.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "hook_executions")]
pub struct Model {
    /// Unique identifier for this execution record
    #[sea_orm(primary_key)]
    pub id: i32,

    /// Session ID from Claude Code
    #[sea_orm(column_type = "Text")]
    pub session_id: String,

    /// Type of hook executed: "SubagentStart" or "SubagentStop"
    #[sea_orm(column_type = "Text")]
    pub hook_type: String,

    /// Agent identifier (e.g., "coder", "tester", "stuck")
    #[sea_orm(column_type = "Text")]
    pub agent_id: String,

    /// Path to the agent's transcript file
    #[sea_orm(column_type = "Text")]
    pub agent_transcript_path: String,

    /// Working directory when the hook was executed
    #[sea_orm(column_type = "Text")]
    pub cwd: String,

    /// Execution status: "success", "failure", or "blocked"
    #[sea_orm(column_type = "Text")]
    pub status: String,

    /// Duration of hook execution in milliseconds
    pub duration_ms: Option<i64>,

    /// Error message if status is "failure"
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: Option<String>,

    /// JSON-encoded payload data from the hook
    #[sea_orm(column_type = "Text", nullable)]
    pub payload_json: Option<String>,

    /// Timestamp when this record was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
