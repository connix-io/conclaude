package logger

import (
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"sync"

	"github.com/connix-io/conclaude/internal/types"
)

var (
	defaultLogger *slog.Logger
	loggerMutex   sync.RWMutex
)

// CreateSessionLogger creates a session-specific logger and sets it as the default.
func CreateSessionLogger(sessionID string, config *types.LoggingConfig) error {
	loggerMutex.Lock()
	defer loggerMutex.Unlock()

	// Determine log level
	logLevel := getLogLevel()

	// Determine output destination
	var output io.Writer
	if os.Getenv("CONCLAUDE_DISABLE_FILE_LOGGING") == "true" || !config.FileLogging {
		output = os.Stderr
	} else {
		file, err := createLogFile(sessionID)
		if err != nil {
			return fmt.Errorf("failed to create log file: %w", err)
		}
		output = file
	}

	// Create JSON handler
	handler := slog.NewJSONHandler(output, &slog.HandlerOptions{
		Level: logLevel,
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			// Format timestamp to match the Rust implementation
			if a.Key == slog.TimeKey {
				a.Value = slog.StringValue(a.Value.Time().UTC().Format("2006-01-02T15:04:05.000Z"))
			}

			return a
		},
	})

	// Create logger with session context
	logger := slog.New(handler).With("session_id", sessionID)
	defaultLogger = logger
	slog.SetDefault(logger)

	return nil
}

// GetLogger returns the current logger instance.
func GetLogger() *slog.Logger {
	loggerMutex.RLock()
	defer loggerMutex.RUnlock()

	if defaultLogger != nil {
		return defaultLogger
	}

	return slog.Default()
}

// getLogLevel converts environment variable to slog.Level.
func getLogLevel() slog.Level {
	logLevel := os.Getenv("CONCLAUDE_LOG_LEVEL")
	switch logLevel {
	case "debug":
		return slog.LevelDebug
	case "info":
		return slog.LevelInfo
	case "warn":
		return slog.LevelWarn
	case "error":
		return slog.LevelError
	default:
		return slog.LevelInfo
	}
}

// createLogFile creates a log file for the session.
func createLogFile(sessionID string) (*os.File, error) {
	tmpDir := os.TempDir()
	logDir := filepath.Join(tmpDir, "conclaude-logs")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create log directory: %w", err)
	}

	logFile := filepath.Join(logDir, fmt.Sprintf("conclaude-%s.log", sessionID))
	file, err := os.OpenFile(logFile, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0644)
	if err != nil {
		return nil, fmt.Errorf("failed to open log file: %w", err)
	}

	return file, nil
}
