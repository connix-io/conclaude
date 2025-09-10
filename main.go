// Package main provides the command-line interface for conclaude.
package main

import (
	"os"

	"github.com/connix-io/conclaude/cmd"
)

const (
	// ExitCodeError represents the exit code for general errors.
	ExitCodeError = 1
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(ExitCodeError)
	}
}
