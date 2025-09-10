package schema

import (
	"testing"
)

func TestValidateConfigAgainstSchema(t *testing.T) {
	tests := []struct {
		name    string
		yaml    string
		wantErr bool
	}{
		{
			name: "valid config",
			yaml: `
stop:
  run: ""
  commands: []
  infinite: false
rules:
  preventRoot: true
  uneditableFiles: []
  toolUsageValidation: []
preToolUse:
  preventAdditions: []
  preventGeneratedFileEdits: true
gitWorktree:
  enabled: false
  autoCreatePR: false
`,
			wantErr: false,
		},
		{
			name: "invalid config with unknown field",
			yaml: `
stop:
  run: ""
unknownField: true
`,
			wantErr: true,
		},
		{
			name: "invalid config with wrong type",
			yaml: `
stop:
  infinite: "not a boolean"
`,
			wantErr: true,
		},
		{
			name:    "empty config",
			yaml:    `{}`,
			wantErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := ValidateConfigAgainstSchema(tt.yaml)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateConfigAgainstSchema() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}