# Makefile for conclaude Go implementation

# Build variables
BINARY_NAME=conclaude
VERSION=0.1.2
BUILD_DIR=bin
GO_FILES=$(shell find . -name "*.go" -not -path "./vendor/*")
LDFLAGS=-ldflags "-X github.com/connix-io/conclaude/cmd.Version=$(VERSION)"

# Default target
.DEFAULT_GOAL := build

# Help target
.PHONY: help
help: ## Display this help message
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Clean build artifacts
.PHONY: clean
clean: ## Remove build artifacts
	@echo "🧹 Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR)
	@rm -f $(BINARY_NAME)
	@echo "✅ Clean complete"

# Format Go code
.PHONY: fmt
fmt: ## Format Go code
	@echo "🔧 Formatting Go code..."
	@go fmt ./...
	@echo "✅ Format complete"

# Lint Go code
.PHONY: lint
lint: fmt ## Lint Go code
	@echo "🔍 Linting Go code..."
	@if command -v golangci-lint >/dev/null 2>&1; then \
		golangci-lint run; \
	else \
		echo "⚠️  golangci-lint not installed, running go vet instead"; \
		go vet ./...; \
	fi
	@echo "✅ Lint complete"

# Run tests
.PHONY: test
test: ## Run tests
	@echo "🧪 Running tests..."
	@go test -v -race -coverprofile=coverage.out ./...
	@echo "✅ Tests complete"

# Test coverage
.PHONY: coverage
coverage: test ## Generate test coverage report
	@echo "📊 Generating coverage report..."
	@go tool cover -html=coverage.out -o coverage.html
	@echo "✅ Coverage report generated: coverage.html"

# Vendor dependencies
.PHONY: vendor
vendor: ## Vendor dependencies
	@echo "📦 Vendoring dependencies..."
	@go mod tidy
	@go mod vendor
	@echo "✅ Vendor complete"

# Download dependencies
.PHONY: deps
deps: ## Download dependencies
	@echo "📥 Downloading dependencies..."
	@go mod download
	@go mod tidy
	@echo "✅ Dependencies downloaded"

# Build the application
.PHONY: build
build: fmt ## Build the application
	@echo "🔨 Building $(BINARY_NAME)..."
	@mkdir -p $(BUILD_DIR)
	@go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME) .
	@echo "✅ Build complete: $(BUILD_DIR)/$(BINARY_NAME)"

# Install the application
.PHONY: install
install: ## Install the application
	@echo "📦 Installing $(BINARY_NAME)..."
	@go install $(LDFLAGS) .
	@echo "✅ Install complete"

# Build for multiple platforms
.PHONY: build-all
build-all: fmt ## Build for multiple platforms
	@echo "🔨 Building for multiple platforms..."
	@mkdir -p $(BUILD_DIR)
	
	@echo "  🐧 Building for Linux amd64..."
	@GOOS=linux GOARCH=amd64 go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-linux-amd64 .
	
	@echo "  🐧 Building for Linux arm64..."
	@GOOS=linux GOARCH=arm64 go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-linux-arm64 .
	
	@echo "  🍎 Building for macOS amd64..."
	@GOOS=darwin GOARCH=amd64 go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-darwin-amd64 .
	
	@echo "  🍎 Building for macOS arm64..."
	@GOOS=darwin GOARCH=arm64 go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-darwin-arm64 .
	
	@echo "  🪟 Building for Windows amd64..."
	@GOOS=windows GOARCH=amd64 go build $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-windows-amd64.exe .
	
	@echo "✅ Multi-platform build complete"

# Run the application
.PHONY: run
run: build ## Build and run the application
	@echo "🚀 Running $(BINARY_NAME)..."
	@./$(BUILD_DIR)/$(BINARY_NAME)

# Development build with race detection
.PHONY: dev
dev: ## Build with development flags (race detection, etc.)
	@echo "🔨 Building $(BINARY_NAME) for development..."
	@mkdir -p $(BUILD_DIR)
	@go build -race $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME)-dev .
	@echo "✅ Development build complete: $(BUILD_DIR)/$(BINARY_NAME)-dev"

# Check for security vulnerabilities
.PHONY: security
security: ## Check for security vulnerabilities
	@echo "🔒 Checking for security vulnerabilities..."
	@if command -v govulncheck >/dev/null 2>&1; then \
		govulncheck ./...; \
	else \
		echo "⚠️  govulncheck not installed, run: go install golang.org/x/vuln/cmd/govulncheck@latest"; \
	fi
	@echo "✅ Security check complete"

# Generate documentation
.PHONY: docs
docs: ## Generate documentation
	@echo "📚 Generating documentation..."
	@if command -v godoc >/dev/null 2>&1; then \
		echo "📖 Starting godoc server at http://localhost:6060"; \
		godoc -http=:6060; \
	else \
		echo "⚠️  godoc not installed, run: go install golang.org/x/tools/cmd/godoc@latest"; \
	fi

# Full CI pipeline
.PHONY: ci
ci: clean deps fmt lint test security build ## Run full CI pipeline

# Show build info
.PHONY: version
version: ## Show version information
	@echo "$(BINARY_NAME) version $(VERSION)"
	@go version
	@echo "Build target: $(BUILD_DIR)/$(BINARY_NAME)"

# Docker build (if Dockerfile exists)
.PHONY: docker
docker: ## Build Docker image
	@if [ -f Dockerfile ]; then \
		echo "🐳 Building Docker image..."; \
		docker build -t $(BINARY_NAME):$(VERSION) .; \
		docker build -t $(BINARY_NAME):latest .; \
		echo "✅ Docker build complete"; \
	else \
		echo "❌ Dockerfile not found"; \
		exit 1; \
	fi