## 1. Implementation

- [ ] 1.1 Update `execute_stop_commands` function signature to accept `config_dir: &Path` parameter
- [ ] 1.2 Add `.current_dir(config_dir)` to the `TokioCommand` in `execute_stop_commands`
- [ ] 1.3 Add `.env("CONCLAUDE_CONFIG_DIR", config_dir)` to the `TokioCommand` in `execute_stop_commands`
- [ ] 1.4 Update `execute_subagent_stop_commands` function signature to accept `config_dir: &Path` parameter
- [ ] 1.5 Add `.current_dir(config_dir)` to the `TokioCommand` in `execute_subagent_stop_commands`
- [ ] 1.6 Update `build_subagent_env_vars` to accept `config_dir: &Path` and add `CONCLAUDE_CONFIG_DIR` to the HashMap
- [ ] 1.7 Update `handle_stop` to extract config directory from `config_path` and pass to `execute_stop_commands`
- [ ] 1.8 Update `handle_subagent_stop` to extract config directory from `config_path`, pass to `build_subagent_env_vars` and `execute_subagent_stop_commands`

## 2. Testing

- [ ] 2.1 Add unit test verifying commands receive correct working directory
- [ ] 2.2 Add unit test verifying `CONCLAUDE_CONFIG_DIR` environment variable is set correctly
- [ ] 2.3 Add integration test with config file in parent directory
- [ ] 2.4 Verify existing tests still pass with `cargo test`

## 3. Validation

- [ ] 3.1 Run `cargo clippy` to ensure no new warnings
- [ ] 3.2 Run `cargo fmt --check` to ensure formatting
- [ ] 3.3 Manual verification: run conclaude from subdirectory with config in parent
