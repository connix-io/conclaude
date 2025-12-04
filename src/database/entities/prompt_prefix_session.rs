use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Prompt prefix session record entity
///
/// Stores the initial prompt (first 100 chars) for each session
/// and tracks the message queue position for prompt prefix blocking.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "prompt_prefix_sessions")]
pub struct Model {
    /// Session ID from Claude Code (primary key)
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub session_id: String,

    /// First 100 characters of the initial prompt
    #[sea_orm(column_type = "Text")]
    pub initial_prompt: String,

    /// Current position in the message queue (0-indexed)
    pub queue_position: i32,

    /// Number of times remaining for the current message
    pub times_remaining: i32,

    /// Timestamp when this record was created
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Timestamp when this record was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
