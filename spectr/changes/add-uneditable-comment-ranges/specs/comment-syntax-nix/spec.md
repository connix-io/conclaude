# comment-syntax-nix Specification

## Purpose

Define Nix-specific comment syntax detection for uneditable range markers, supporting line comments (`#`) and block comments (`/* */`).

## ADDED Requirements

### Requirement: Line Comment Detection

The system SHALL recognize uneditable markers within Nix line comments (`#`).

#### Scenario: Single-line comment with marker

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed for uneditable markers
- **THEN** the start marker SHALL be detected
- **AND** the marker line SHALL be recorded as the start of a protected range

#### Scenario: Line comment with trailing content

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start --> Auto-generated package definition
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected (trailing content ignored)
- **AND** the range SHALL begin at this line

### Requirement: Block Comment Detection

The system SHALL recognize uneditable markers within Nix block comments (`/* */`).

#### Scenario: Block comment with marker

- **GIVEN** a Nix file with:
  ```nix
  /* <!-- conclaude-uneditable:start --> */
  ```
- **WHEN** the file is parsed
- **THEN** the start marker SHALL be detected within the block comment
- **AND** the range SHALL begin at this line

#### Scenario: Multi-line block comment with marker

- **GIVEN** a Nix file with:
  ```nix
  /*
   * <!-- conclaude-uneditable:start -->
   * Auto-generated derivation
   */
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be detected on line 2
- **AND** the range SHALL begin at line 2

### Requirement: File Extension Mapping

The system SHALL detect Nix files by their file extension and apply Nix comment syntax rules.

#### Scenario: .nix file extension

- **GIVEN** a file named "default.nix"
- **WHEN** the file is processed for uneditable ranges
- **THEN** Nix comment syntax rules SHALL be applied
- **AND** markers within `#` or `/* */` comments SHALL be detected

#### Scenario: .nix file with markers

- **GIVEN** a file "package.nix" containing:
  ```nix
  { pkgs }:

  # <!-- conclaude-uneditable:start -->
  pkgs.stdenv.mkDerivation {
    pname = "myapp";
    version = "1.0.0";
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** a protected range from line 3 to line 8 SHALL be created
- **AND** edits overlapping this range SHALL be blocked

### Requirement: Marker Format Validation

The system SHALL only recognize correctly formatted markers within Nix comments.

#### Scenario: Marker in string literal (not detected)

- **GIVEN** a Nix file with:
  ```nix
  description = "# <!-- conclaude-uneditable:start -->";
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string, not comment)
- **AND** no protected range SHALL be created

#### Scenario: Marker in multi-line string (not detected)

- **GIVEN** a Nix file with:
  ```nix
  longDescription = ''
    # <!-- conclaude-uneditable:start -->
    Some description
  '';
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL NOT be detected (inside string)
- **AND** no protected range SHALL be created

#### Scenario: Correctly formatted marker in comment

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker SHALL be recognized
- **AND** a protected range SHALL start at this line

### Requirement: Nested Range Handling

The system SHALL handle nested ranges within Nix expressions.

#### Scenario: Nested attribute set protection

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->  # Line 1
  {
    # <!-- conclaude-uneditable:start -->  # Line 3
    buildInputs = [ pkgs.gcc pkgs.make ];
    # <!-- conclaude-uneditable:end -->  # Line 5

    meta = { };
  }
  # <!-- conclaude-uneditable:end -->  # Line 9
  ```
- **WHEN** the file is parsed
- **THEN** two ranges SHALL be created: (1-9) and (3-5)
- **AND** both ranges SHALL be marked as protected
- **AND** edits overlapping either range SHALL be blocked

### Requirement: Edge Case Handling

The system SHALL gracefully handle edge cases in Nix comment syntax.

#### Scenario: Inline comment after code

- **GIVEN** a Nix file with:
  ```nix
  version = "1.0";  # <!-- conclaude-uneditable:start -->
  name = "app";
  tag = "latest";  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** both markers SHALL be detected (inline comments are valid)
- **AND** a protected range from line 1 to line 3 SHALL be created

#### Scenario: Marker at file start

- **GIVEN** a Nix file with marker at line 1:
  ```nix
  # <!-- conclaude-uneditable:start -->
  { pkgs }: pkgs.hello
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL start at line 1
- **AND** the range SHALL end at line 3

#### Scenario: Marker at file end

- **GIVEN** a Nix file ending with:
  ```nix
  {
    lastAttr = "value";
  }
  # <!-- conclaude-uneditable:start -->
  # Generated metadata
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL end at the last line of the file
- **AND** the range SHALL be valid

#### Scenario: Empty lines between markers

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->


  {
    protected = true;
  }


  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the range SHALL include all lines between markers (including empty lines)
- **AND** edits within the range SHALL be blocked

### Requirement: Nix Expression Compatibility

The system SHALL correctly handle Nix language constructs with markers.

#### Scenario: Let binding with marker

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->
  let
    version = "1.0.0";
    name = "mypackage";
  in
  # <!-- conclaude-uneditable:end -->
  { inherit version name; }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 6 SHALL be created

#### Scenario: Derivation with marker

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->
  pkgs.stdenv.mkDerivation rec {
    pname = "generated-app";
    version = "0.1.0";

    src = fetchurl {
      url = "https://example.com/source.tar.gz";
      sha256 = "abc123...";
    };
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 11 SHALL be created

### Requirement: NixOS Module Compatibility

The system SHALL handle markers in NixOS configuration modules.

#### Scenario: Module options with marker

- **GIVEN** a Nix file with:
  ```nix
  { config, pkgs, ... }:

  # <!-- conclaude-uneditable:start -->
  {
    services.nginx.enable = true;
    services.nginx.virtualHosts."example.com" = {
      root = "/var/www";
    };
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 3 SHALL be detected
- **AND** a protected range from line 3 to line 10 SHALL be created

#### Scenario: Import with marker

- **GIVEN** a Nix file with:
  ```nix
  { config, pkgs, ... }:

  {
    # <!-- conclaude-uneditable:start -->
    imports = [
      ./hardware-configuration.nix
      ./generated-config.nix
    ];
    # <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 9 SHALL be created

### Requirement: Nix Flake Compatibility

The system SHALL handle markers in Nix flake files.

#### Scenario: Flake outputs with marker

- **GIVEN** a flake.nix file with:
  ```nix
  {
    description = "My flake";

    # <!-- conclaude-uneditable:start -->
    outputs = { self, nixpkgs }: {
      packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.hello;
    };
    # <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 4 SHALL be detected
- **AND** a protected range from line 4 to line 8 SHALL be created

#### Scenario: Flake inputs with marker

- **GIVEN** a flake.nix file with:
  ```nix
  {
    # <!-- conclaude-uneditable:start -->
    inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
      home-manager.url = "github:nix-community/home-manager";
    };
    # <!-- conclaude-uneditable:end -->

    outputs = { ... }: { };
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 7 SHALL be created

### Requirement: Attribute Path Handling

The system SHALL handle markers around attribute paths and sets.

#### Scenario: Nested attribute set with marker

- **GIVEN** a Nix file with:
  ```nix
  {
    # <!-- conclaude-uneditable:start -->
    systemPackages = with pkgs; [
      vim
      git
      htop
    ];
    # <!-- conclaude-uneditable:end -->
  }
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 2 SHALL be detected
- **AND** a protected range from line 2 to line 8 SHALL be created

#### Scenario: Function with marker

- **GIVEN** a Nix file with:
  ```nix
  # <!-- conclaude-uneditable:start -->
  { stdenv, fetchurl }:

  stdenv.mkDerivation {
    name = "generated";
  }
  # <!-- conclaude-uneditable:end -->
  ```
- **WHEN** the file is parsed
- **THEN** the marker on line 1 SHALL be detected
- **AND** a protected range from line 1 to line 7 SHALL be created

### Requirement: Performance Characteristics

The system SHALL efficiently parse Nix files for uneditable markers.

#### Scenario: Large Nix file with multiple markers

- **GIVEN** a Nix file with 4,000 lines and 10 protected ranges
- **WHEN** the file is parsed for markers
- **THEN** parsing SHALL complete in under 60ms
- **AND** all 10 ranges SHALL be correctly identified

#### Scenario: Nix file with no markers

- **GIVEN** a Nix file with 2,500 lines and no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL complete quickly (under 35ms)
- **AND** an empty range list SHALL be returned
- **AND** no false positive detections SHALL occur

#### Scenario: File with many comments but no markers

- **GIVEN** a Nix file with 300 comment lines but no uneditable markers
- **WHEN** the file is parsed
- **THEN** parsing SHALL scan all comments efficiently
- **AND** no protected ranges SHALL be created
- **AND** performance SHALL remain acceptable (under 45ms)
