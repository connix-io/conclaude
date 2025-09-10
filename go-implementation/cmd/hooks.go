package cmd

import (
	"github.com/connix-io/conclaude-go/internal/hooks"
	"github.com/spf13/cobra"
)

// preToolUseCmd represents the PreToolUse command
var preToolUseCmd = &cobra.Command{
	Use:   "PreToolUse",
	Short: "Process PreToolUse hook - fired before tool execution",
	Long:  "Process PreToolUse hook - fired before Claude executes any tool. Allows blocking or modifying tool execution before it occurs.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandlePreToolUse)
	},
}

// postToolUseCmd represents the PostToolUse command
var postToolUseCmd = &cobra.Command{
	Use:   "PostToolUse",
	Short: "Process PostToolUse hook - fired after tool execution",
	Long:  "Process PostToolUse hook - fired after Claude executes a tool. Contains both the input and response data for analysis or logging.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandlePostToolUse)
	},
}

// notificationCmd represents the Notification command
var notificationCmd = &cobra.Command{
	Use:   "Notification",
	Short: "Process Notification hook - fired for system notifications",
	Long:  "Process Notification hook - fired when Claude sends system notifications. Used for displaying messages or alerts to the user.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandleNotification)
	},
}

// userPromptSubmitCmd represents the UserPromptSubmit command
var userPromptSubmitCmd = &cobra.Command{
	Use:   "UserPromptSubmit",
	Short: "Process UserPromptSubmit hook - fired when user submits input",
	Long:  "Process UserPromptSubmit hook - fired when user submits input to Claude. Allows processing or validation of user input before Claude processes it.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandleUserPromptSubmit)
	},
}

// sessionStartCmd represents the SessionStart command
var sessionStartCmd = &cobra.Command{
	Use:   "SessionStart",
	Short: "Process SessionStart hook - fired when session begins",
	Long:  "Process SessionStart hook - fired when a new Claude session begins. Allows initialization or setup operations at the start of a conversation.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandleSessionStart)
	},
}

// stopCmd represents the Stop command
var stopCmd = &cobra.Command{
	Use:   "Stop",
	Short: "Process Stop hook - fired when session terminates",
	Long:  "Process Stop hook - fired when a Claude session is terminating. Allows for cleanup operations or final processing before session ends.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandleStop)
	},
}

// subagentStopCmd represents the SubagentStop command
var subagentStopCmd = &cobra.Command{
	Use:   "SubagentStop",
	Short: "Process SubagentStop hook - fired when subagent completes",
	Long:  "Process SubagentStop hook - fired when a Claude subagent terminates. Subagents are spawned for complex tasks and this fires when they complete.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandleSubagentStop)
	},
}

// preCompactCmd represents the PreCompact command
var preCompactCmd = &cobra.Command{
	Use:   "PreCompact",
	Short: "Process PreCompact hook - fired before transcript compaction",
	Long:  "Process PreCompact hook - fired before transcript compaction occurs. Transcript compaction reduces conversation history size to manage context limits.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return hooks.HandleHookResult(hooks.HandlePreCompact)
	},
}