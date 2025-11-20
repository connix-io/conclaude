# comment-syntax-r Specification

## Purpose

Define R-specific comment syntax detection for uneditable range markers, supporting line comments (`#`) in R scripts.

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within R line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Indented comment with marker

- **GIVEN** an R file with:
  ```r
  my_function <- function(x) {
    # <!-- conclaude-uneditable:start -->
    result <- x * 2
    return(result)
    # <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected regardless of indentation
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: Comment with leading whitespace

- **GIVEN** an R file with:
  ```r
      # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected after trimming leading whitespace
- **AND** the range SHALL begin at this line

### Requirement: File Extension Mapping

The system SHALL detect R files by their file extension and apply R comment syntax rules.

#### Scenario: .r file extension (lowercase)

- **GIVEN** a file named "analysis.r"
- **WHEN** the file is processed for uneditable ranges
- **THEN** R comment syntax rules SHALL be applied
- **AND** markers within `#` comments SHALL be detected

#### Scenario: .R file extension (uppercase)

- **GIVEN** a file named "Analysis.R"
- **WHEN** the file is processed for uneditable ranges
- **THEN** R comment syntax rules SHALL be applied
- **AND** markers SHALL be detected correctly

#### Scenario: .Rmd file extension (R Markdown)

- **GIVEN** a file named "report.Rmd"
- **WHEN** the file is processed for uneditable ranges
- **THEN** R comment syntax rules SHALL be applied to R code chunks
- **AND** markers in `#` comments SHALL be detected

#### Scenario: .R file with markers

- **GIVEN** a file "generated_model.R" containing:
  ```r
  # <!-- conclaude-uneditable:start -->
  # Auto-generated statistical model
  model <- lm(y ~ x1 + x2, data = dataset)
  coefficients <- coef(model)
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 1 to line 5 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within R comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** an R file with:
  ```r
  message <- "# <!-- conclaude-uneditable:start -->"
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in character vector (not detected)

- **GIVEN** an R file with:
  ```r
  comments <- c("# <!-- conclaude-uneditable:start -->", "other text")
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within R code structure.

#### Scenario: Nested function protection

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->  # Line 1
  process_data <- function(df) {
    # <!-- conclaude-uneditable:start -->  # Line 3
    validate <- function(x) {
      !is.na(x) && x > 0
    }
    # <!-- conclaude-uneditable:end -->  # Line 7

    filtered <- df[sapply(df$value, validate), ]
    return(filtered)
  }
  # <!-- conclaude-uneditable:end -->  # Line 12
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-12) and (3-7)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in R comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** an R file with:
  ```r
  x <- 5  # <!-- conclaude-uneditable:start -->
  y <- 10
  z <- 15  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** an R file with marker at line 1:
  ```r
  # <!-- conclaude-uneditable:start -->
  library(tidyverse)
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** an R file ending with:
  ```r
  final_function <- function() {
    return(TRUE)
  }
  # <!-- conclaude-uneditable:start -->
  # Generated footer
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->


  protected_calc <- function(x) {
    x^2
  }


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: R Package and Library Compatibility

The system SHALL correctly handle R files with library imports and package declarations.

#### Scenario: File with library statements

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  library(dplyr)
  library(ggplot2)
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

#### Scenario: File with require statements

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  require(tidyr)
  require(stringr, quietly = TRUE)
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

### Requirement: R-Specific Syntax Handling

The system SHALL handle R-specific patterns and constructs.

#### Scenario: Data frame manipulation with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  df <- data.frame(
    id = 1:10,
    value = rnorm(10)
  )
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Function definition with roxygen2 comments

- **GIVEN** an R file with:
  ```r
  #' @title Calculate Sum
  #' @param x numeric vector
  #' @return numeric sum
  # <!-- conclaude-uneditable:start -->
  calculate_sum <- function(x) {
    sum(x, na.rm = TRUE)
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 8 SHALL be created

#### Scenario: S3 method with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  print.custom_class <- function(x, ...) {
    cat("Custom object:\n")
    print(unclass(x))
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Pipe operator chain with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  result <- data %>%
    filter(x > 0) %>%
    mutate(y = x * 2) %>%
    summarize(total = sum(y))
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Formula with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  model_formula <- y ~ x1 + x2 + x1:x2
  fit <- lm(model_formula, data = df)
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 4 SHALL be created

### Requirement: Roxygen2 Documentation Compatibility

The system SHALL correctly handle R files with roxygen2 documentation blocks.

#### Scenario: Roxygen2 block with marker

- **GIVEN** an R file with:
  ```r
  # <!-- conclaude-uneditable:start -->
  #' @title Process Dataset
  #' @description Auto-generated processing function
  #' @param data input data frame
  #' @export
  process_dataset <- function(data) {
    # processing logic
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 9 SHALL be created

#### Scenario: Roxygen2 with examples

- **GIVEN** an R file with:
  ```r
  #' @examples
  #' result <- calculate(10)
  # <!-- conclaude-uneditable:start -->
  calculate <- function(x) {
    x * 2
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 7 SHALL be created

### Requirement: R Markdown Compatibility

The system SHALL handle markers in R Markdown files within R code chunks.

#### Scenario: R code chunk with marker

- **GIVEN** an R Markdown file with:
  ````markdown
  ```{r}
  # <!-- conclaude-uneditable:start -->
  library(ggplot2)
  plot_data <- ggplot(mtcars, aes(x = mpg, y = hp))
  # <!-- conclaude-uneditable:end -->
  ```
  ````
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 5 SHALL be created

#### Scenario: Multiple code chunks

- **GIVEN** an R Markdown file with multiple chunks containing markers
- **WHEN** the file is parsed
- **THEN** all markers in R comments SHALL be detected
- **AND** each protected range SHALL be correctly identified

### Requirement: Performance Characteristics

The system SHALL efficiently parse R files for uneditable markers.

#### Scenario: Large R file with multiple markers

- **GIVEN** an R file with 5,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 50ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: R file with no markers

- **GIVEN** an R file with 2,000 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 30ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many roxygen2 comments but no markers

- **GIVEN** an R file with 400 lines of roxygen2 documentation but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 40ms)
