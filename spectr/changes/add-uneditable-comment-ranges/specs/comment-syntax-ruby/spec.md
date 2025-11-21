# comment-syntax-ruby Specification

## Purpose

Define Ruby-specific comment syntax detection for uneditable range markers, supporting line comments (`#`) and block comments (`=begin`/`=end`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Ruby line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Indented comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  class User
    # <!-- conclaude-uneditable:start -->
    def authenticate
      true
    end
    # <!-- conclaude-uneditable:end -->
  end
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range from line 2 to line 6 SHALL be created

#### Scenario: Comment with leading whitespace

- **GIVEN** a Ruby file with:
  ```ruby
      # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Ruby block comments (`=begin`/`=end`).

#### Scenario: Block comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  =begin
  <!-- conclaude-uneditable:start -->
  Auto-generated code below
  =end
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

#### Scenario: Multi-line block comment with end marker

- **GIVEN** a Ruby file with:
  ```ruby
  =begin
  Protected section
  <!-- conclaude-uneditable:end -->
  =end
  ```
- **WHEN** the file is parsed
- **THEN** the end marker SHALL be detected on line 3
- **AND** the range SHALL end at line 3

### Requirement: File Extension Mapping

The system SHALL detect Ruby files by their file extension and apply Ruby comment syntax rules.

#### Scenario: .rb file extension

- **GIVEN** a file named "user.rb"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Ruby comment syntax rules SHALL be applied
- **AND** markers within `#` or `=begin`/`=end` comments SHALL be detected

#### Scenario: .rake file extension

- **GIVEN** a file named "tasks.rake"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Ruby comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .gemspec file extension

- **GIVEN** a file named "myapp.gemspec"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Ruby comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: Rakefile (no extension)

- **GIVEN** a file named "Rakefile"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Ruby comment syntax rules SHOULD be applied if file naming is detected
- **OR** no detection occurs if file extension is the only method (acceptable fallback)

#### Scenario: .rb file with markers

- **GIVEN** a file "generated_model.rb" containing:
  ```ruby
  # <!-- conclaude-uneditable:start -->
  # Auto-generated ActiveRecord model
  class User < ApplicationRecord
    has_many :posts
  end
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 6 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Ruby comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Ruby file with:
  ```ruby
  message = "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in heredoc (not detected)

- **GIVEN** a Ruby file with:
  ```ruby
  doc = <<~HEREDOC
    # <!-- conclaude-uneditable:start -->
    Some text
  HEREDOC
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside heredoc)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Ruby file with:
  ```ruby
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Ruby code structure.

#### Scenario: Nested method protection

- **GIVEN** a Ruby file with:
  ```ruby
  # <!-- conclaude-uneditable:start -->  # Line 1
  class AuthService
    # <!-- conclaude-uneditable:start -->  # Line 3
    def verify_token(token)
      true
    end
    # <!-- conclaude-uneditable:end -->  # Line 7
  end
  # <!-- conclaude-uneditable:end -->  # Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

#### Scenario: Nested ranges with mixed indentation

- **GIVEN** a Ruby file with nested ranges at different indentation levels
- **WHEN** the file is parsed
- **THEN** all markers SHALL be detected regardless of indentation
- **AND** ranges SHALL be correctly paired by nesting order (not indentation)

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Ruby comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a Ruby file with:
  ```ruby
  x = 5  # <!-- conclaude-uneditable:start -->
  y = 10
  z = 15  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a Ruby file with marker at line 1:
  ```ruby
  # <!-- conclaude-uneditable:start -->
  require 'rails'
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a Ruby file ending with:
  ```ruby
  def last_method
  end
  # <!-- conclaude-uneditable:start -->
  # Protected footer
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a Ruby file with:
  ```ruby
  # <!-- conclaude-uneditable:start -->


  def protected_method
  end


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Shebang and Encoding Compatibility

The system SHALL correctly handle Ruby files with shebang lines and encoding declarations.

#### Scenario: File with shebang

- **GIVEN** a Ruby file starting with:
  ```ruby
  #!/usr/bin/env ruby
  # <!-- conclaude-uneditable:start -->
  require 'json'
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected (after shebang)
- **AND** a protected range from line 2 to line 4 SHALL be created

#### Scenario: File with encoding declaration

- **GIVEN** a Ruby file with:
  ```ruby
  # encoding: utf-8
  # <!-- conclaude-uneditable:start -->
  class User
  end
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected (after encoding)
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: Magic comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  # frozen_string_literal: true
  # <!-- conclaude-uneditable:start -->
  module Generated
  end
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 5 SHALL be created

### Requirement: Ruby-Specific Syntax Handling

The system SHALL handle Ruby-specific comment patterns and idioms.

#### Scenario: RDoc comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  ##
  # <!-- conclaude-uneditable:start -->
  # Auto-generated RDoc
  ##
  class Generated
  end
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

#### Scenario: YARD comment with marker

- **GIVEN** a Ruby file with:
  ```ruby
  # <!-- conclaude-uneditable:start -->
  # @param [String] token
  # @return [Boolean]
  def verify(token)
    true
  end
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Ruby files for uneditable markers.

#### Scenario: Large Ruby file with multiple markers

- **GIVEN** a Ruby file with 5,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Ruby file with no markers

- **GIVEN** a Ruby file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a Ruby file with 300 comment lines (RDoc, YARD, etc.) but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 40ms)
