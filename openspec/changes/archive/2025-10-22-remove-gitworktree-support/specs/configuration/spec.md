## REMOVED Requirements
### Requirement: Git Worktree Configuration Support
**Reason**: The gitWorktree configuration adds unnecessary complexity without providing clear value to users. This functionality will be completely removed to simplify the configuration schema.

**Migration**: Users with gitWorktree configurations should remove these sections from their .conclaude.yaml files. The application will ignore unknown configuration fields, so existing configurations will not cause runtime errors.

#### Scenario: Configuration loading without gitWorktree
- **WHEN** a ConclaudeConfig is loaded from YAML or JSON
- **THEN** the configuration MUST NOT include gitWorktree field
- **AND** the schema MUST validate without gitWorktree properties

#### Scenario: Default configuration generation
- **WHEN** generate_default_config() is called
- **THEN** the output MUST NOT contain gitWorktree section
- **AND** all other configuration sections MUST remain intact

#### Scenario: JSON schema validation
- **WHEN** JSON schema validation is performed
- **THEN** the schema MUST NOT define GitWorktreeConfig
- **AND** MUST NOT reference gitWorktree properties
- **AND** MUST reject gitWorktree fields as unknown properties