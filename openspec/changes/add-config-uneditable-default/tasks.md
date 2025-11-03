# Implementation Tasks

## 1. Update Default Configuration

- [ ] 1.1 Modify `src/default-config.yaml` to add `.conclaude.yml` and `.conclaude.yaml` to `rules.uneditableFiles`
- [ ] 1.2 Update comments in the `rules.uneditableFiles` section to explain the config file protection
- [ ] 1.3 Ensure the example format matches existing patterns in the file

## 2. Testing

- [ ] 2.1 Verify that `conclaude init` generates config with the new defaults
- [ ] 2.2 Test that the generated config parses correctly
- [ ] 2.3 Verify that file protection works when uneditableFiles includes `.conclaude.yml`

## 3. Documentation

- [ ] 3.1 Update README if necessary to mention default protection of config files
- [ ] 3.2 Verify schema documentation reflects the usage

## 4. Validation

- [ ] 4.1 Run `openspec validate add-config-uneditable-default --strict`
- [ ] 4.2 Run tests to ensure no regressions
- [ ] 4.3 Verify the change builds successfully
