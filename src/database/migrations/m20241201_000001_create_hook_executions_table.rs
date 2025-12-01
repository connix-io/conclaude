use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create hook_executions table
        manager
            .create_table(
                Table::create()
                    .table(HookExecutions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HookExecutions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::SessionId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::HookType)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::AgentId)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::AgentTranscriptPath)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::Cwd)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::Status)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::DurationMs)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::PayloadJson)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(HookExecutions::CreatedAt)
                            .text()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on session_id for fast lookups by session
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hook_executions_session_id")
                    .table(HookExecutions::Table)
                    .col(HookExecutions::SessionId)
                    .to_owned(),
            )
            .await?;

        // Create index on agent_id for fast lookups by agent
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hook_executions_agent_id")
                    .table(HookExecutions::Table)
                    .col(HookExecutions::AgentId)
                    .to_owned(),
            )
            .await?;

        // Create index on hook_type for filtering by hook type
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hook_executions_hook_type")
                    .table(HookExecutions::Table)
                    .col(HookExecutions::HookType)
                    .to_owned(),
            )
            .await?;

        // Create composite index on session_id and agent_id for common query pattern
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_hook_executions_session_agent")
                    .table(HookExecutions::Table)
                    .col(HookExecutions::SessionId)
                    .col(HookExecutions::AgentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop all indexes
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hook_executions_session_agent")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hook_executions_hook_type")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hook_executions_agent_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_hook_executions_session_id")
                    .to_owned(),
            )
            .await?;

        // Drop the table
        manager
            .drop_table(Table::drop().table(HookExecutions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HookExecutions {
    #[sea_orm(iden = "hook_executions")]
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "session_id")]
    SessionId,
    #[sea_orm(iden = "hook_type")]
    HookType,
    #[sea_orm(iden = "agent_id")]
    AgentId,
    #[sea_orm(iden = "agent_transcript_path")]
    AgentTranscriptPath,
    #[sea_orm(iden = "cwd")]
    Cwd,
    #[sea_orm(iden = "status")]
    Status,
    #[sea_orm(iden = "duration_ms")]
    DurationMs,
    #[sea_orm(iden = "error_message")]
    ErrorMessage,
    #[sea_orm(iden = "payload_json")]
    PayloadJson,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}
