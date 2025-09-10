# Multi-stage build for conclaude Go implementation
FROM golang:1.22-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git make

# Set working directory
WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN make build

# Final stage - minimal runtime image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1000 conclaude && \
    adduser -D -u 1000 -G conclaude conclaude

# Set working directory
WORKDIR /home/conclaude

# Copy binary from builder stage
COPY --from=builder /app/bin/conclaude /usr/local/bin/conclaude

# Set ownership
RUN chown conclaude:conclaude /usr/local/bin/conclaude

# Switch to non-root user
USER conclaude

# Set entrypoint
ENTRYPOINT ["conclaude"]

# Default command shows help
CMD ["--help"]