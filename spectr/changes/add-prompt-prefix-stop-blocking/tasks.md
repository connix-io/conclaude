## 1. Database Schema

- [ ] 1.1 Create SeaORM migration for `prompt_prefix_sessions` table
- [ ] 1.2 Add columns: `session_id` (PK), `initial_prompt` (VARCHAR 100), `queue_position` (INT), `times_remaining` (INT)
- [ ] 1.3 Add columns: `created_at`, `updated_at` (timestamps)
- [ ] 1.4 Generate SeaORM entity for `prompt_prefix_sessions`

## 2. Configuration Schema

- [ ] 2.1 Add `PromptPrefixBlockingMessage` struct with `text: String` and `times: Option<u32>` (default 1)
- [ ] 2.2 Add `PromptPrefixBlockingConfig` struct with `prefixes: Vec<String>` (glob patterns) and `messages: Vec<PromptPrefixBlockingMessage>`
- [ ] 2.3 Add `prompt_prefix_blocking: Option<PromptPrefixBlockingConfig>` field to `StopConfig`
- [ ] 2.4 Update JSON schema generation for new config types
- [ ] 2.5 Add config examples to `default-config.yaml` (commented out)

## 3. UserPromptSubmit Hook Enhancement

- [ ] 3.1 In `handle_user_prompt_submit()`, check if `prompt_prefix_blocking` is configured
- [ ] 3.2 Check if `database.enabled` is true; if false, skip prefix blocking with warning log
- [ ] 3.3 Query database to check if session already has a stored prompt
- [ ] 3.4 If first prompt, store first 100 chars to database with initial queue_position=0, times_remaining from first message
- [ ] 3.5 Ensure idempotency - only insert if no existing record for session_id

## 4. Stop Hook Integration

- [ ] 4.1 In `handle_stop()`, check if `prompt_prefix_blocking` is configured
- [ ] 4.2 Check if `database.enabled` is true; if false, skip prefix blocking
- [ ] 4.3 Query database for session record using session_id from payload
- [ ] 4.4 If no record exists, skip prefix blocking (allow normal stop flow)
- [ ] 4.5 Match stored initial prompt against configured glob patterns (case-sensitive)
- [ ] 4.6 If match found and messages remain, block stop and return current message
- [ ] 4.7 Decrement `times_remaining` in database, advance `queue_position` when exhausted
- [ ] 4.8 Allow stop to proceed when all messages exhausted (queue_position >= messages.len())

## 5. Testing

- [ ] 5.1 Add unit tests for glob pattern matching on prompt prefixes (first 100 chars)
- [ ] 5.2 Add unit tests for message queue iteration with `times` decrementing
- [ ] 5.3 Add unit tests for database CRUD operations on `prompt_prefix_sessions`
- [ ] 5.4 Add integration tests for full stop-blocking flow with database persistence
- [ ] 5.5 Test edge cases: no match, empty messages, times=0, process restart resume

## 6. Documentation

- [ ] 6.1 Update README with `promptPrefixBlocking` configuration examples
- [ ] 6.2 Document glob pattern syntax for prefix matching (case-sensitive, first 100 chars)
- [ ] 6.3 Document message queue behavior and `times` field semantics
- [ ] 6.4 Document database table structure for debugging/audit
