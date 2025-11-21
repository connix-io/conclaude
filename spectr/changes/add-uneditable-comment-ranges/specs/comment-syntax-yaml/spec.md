# comment-syntax-yaml Specification

## Purpose

Define YAML-specific comment syntax detection for uneditable range markers, supporting line comments (`#`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within YAML line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Indented comment with marker

- **GIVEN** a YAML file with:
  ```yaml
  server:
    # <!-- conclaude-uneditable:start -->
    host: localhost
    port: 8080
    # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: Comment with leading whitespace

- **GIVEN** a YAML file with:
  ```yaml
      # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect YAML files by their file extension and apply YAML comment syntax rules.

#### Scenario: .yaml file extension

- **GIVEN** a file named "config.yaml"
- **WHEN** the file is processed for uneditable ranges
- **THEN** YAML comment syntax rules SHALL be applied
- **AND** markers within `#` comments SHALL be detected

#### Scenario: .yml file extension

- **GIVEN** a file named "docker-compose.yml"
- **WHEN** the file is processed for uneditable ranges
- **THEN** YAML comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .yaml file with markers

- **GIVEN** a file "application.yaml" containing:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  # Auto-generated database configuration
  database:
    driver: postgresql
    host: localhost
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within YAML comments.

#### Scenario: Marker in string value (not detected)

- **GIVEN** a YAML file with:
  ```yaml
  message: "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in multi-line string (not detected)

- **GIVEN** a YAML file with:
  ```yaml
  description: |
    # <!-- conclaude-uneditable:start -->
    Some description
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within YAML structure.

#### Scenario: Nested mapping protection

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->  # Line 1
  services:
    # <!-- conclaude-uneditable:start -->  # Line 3
    database:
      host: localhost
      port: 5432
    # <!-- conclaude-uneditable:end -->  # Line 7
    cache:
      enabled: true
  # <!-- conclaude-uneditable:end -->  # Line 10
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-10) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in YAML comment syntax.

#### Scenario: Inline comment after value

- **GIVEN** a YAML file with:
  ```yaml
  name: app  # <!-- conclaude-uneditable:start -->
  version: 1.0
  tag: latest  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a YAML file with marker at line 1:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  version: "3.8"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a YAML file ending with:
  ```yaml
  lastProperty: value
  # <!-- conclaude-uneditable:start -->
  # Generated metadata
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->


  protected:
    enabled: true


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Docker Compose Compatibility

The system SHALL correctly handle YAML files used in Docker Compose.

#### Scenario: Docker Compose service with marker

- **GIVEN** a docker-compose.yml file with:
  ```yaml
  version: '3.8'
  services:
    # <!-- conclaude-uneditable:start -->
    database:
      image: postgres:14
      environment:
        POSTGRES_PASSWORD: secret
    # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 8 SHALL be created

#### Scenario: Docker Compose volumes with marker

- **GIVEN** a docker-compose.yml file with:
  ```yaml
  services:
    app:
      image: myapp:latest
  # <!-- conclaude-uneditable:start -->
  volumes:
    data:
      driver: local
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 8 SHALL be created

### Requirement: Kubernetes Compatibility

The system SHALL handle markers in Kubernetes YAML manifests.

#### Scenario: Kubernetes deployment with marker

- **GIVEN** a Kubernetes manifest with:
  ```yaml
  apiVersion: apps/v1
  kind: Deployment
  # <!-- conclaude-uneditable:start -->
  metadata:
    name: nginx-deployment
    labels:
      app: nginx
  # <!-- conclaude-uneditable:end -->
  spec:
    replicas: 3
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 8 SHALL be created

#### Scenario: Kubernetes service with marker

- **GIVEN** a Kubernetes manifest with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  apiVersion: v1
  kind: Service
  metadata:
    name: my-service
  spec:
    selector:
      app: MyApp
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

### Requirement: YAML-Specific Syntax Handling

The system SHALL handle YAML-specific patterns and constructs.

#### Scenario: Sequence with marker

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  items:
    - name: item1
      value: 10
    - name: item2
      value: 20
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

#### Scenario: Anchor and alias with marker

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  defaults: &defaults
    timeout: 30
    retries: 3
  # <!-- conclaude-uneditable:end -->

  service:
    <<: *defaults
    port: 8080
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Multi-document YAML with marker

- **GIVEN** a YAML file with multiple documents:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  name: doc1
  ---
  name: doc2
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

#### Scenario: Literal scalar with marker

- **GIVEN** a YAML file with:
  ```yaml
  # <!-- conclaude-uneditable:start -->
  script: |
    #!/bin/bash
    echo "Hello"
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 5 SHALL be created

### Requirement: GitHub Actions Compatibility

The system SHALL handle markers in GitHub Actions workflow files.

#### Scenario: Workflow with marker

- **GIVEN** a .github/workflows/ci.yml file with:
  ```yaml
  name: CI
  on: [push, pull_request]

  # <!-- conclaude-uneditable:start -->
  jobs:
    build:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v3
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 10 SHALL be created

#### Scenario: Matrix strategy with marker

- **GIVEN** a GitHub Actions workflow with:
  ```yaml
  jobs:
    test:
      # <!-- conclaude-uneditable:start -->
      strategy:
        matrix:
          node-version: [14, 16, 18]
      # <!-- conclaude-uneditable:end -->
      runs-on: ubuntu-latest
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 7 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse YAML files for uneditable markers.

#### Scenario: Large YAML file with multiple markers

- **GIVEN** a YAML file with 3,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: YAML file with no markers

- **GIVEN** a YAML file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a YAML file with 300 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 40ms)
