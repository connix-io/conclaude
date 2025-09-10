package main

import (
	"os"

	"github.com/connix-io/conclaude/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
