use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create prompt_prefix_sessions table
        manager
            .create_table(
                Table::create()
                    .table(PromptPrefixSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PromptPrefixSessions::SessionId)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PromptPrefixSessions::InitialPrompt)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PromptPrefixSessions::QueuePosition)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(PromptPrefixSessions::TimesRemaining)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(PromptPrefixSessions::CreatedAt)
                            .text()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PromptPrefixSessions::UpdatedAt)
                            .text()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the table
        manager
            .drop_table(Table::drop().table(PromptPrefixSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PromptPrefixSessions {
    #[sea_orm(iden = "prompt_prefix_sessions")]
    Table,
    #[sea_orm(iden = "session_id")]
    SessionId,
    #[sea_orm(iden = "initial_prompt")]
    InitialPrompt,
    #[sea_orm(iden = "queue_position")]
    QueuePosition,
    #[sea_orm(iden = "times_remaining")]
    TimesRemaining,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
