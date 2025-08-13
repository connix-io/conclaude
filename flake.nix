/**
# Go Development Shell Template

## Description
Complete Go development environment with modern tooling for building, testing,
and maintaining Go applications. Includes the Go toolchain, linting, formatting,
live reloading, and testing utilities for productive Go development.

## Platform Support
- ✅ x86_64-linux
- ✅ aarch64-linux (ARM64 Linux)
- ✅ x86_64-darwin (Intel macOS)
- ✅ aarch64-darwin (Apple Silicon macOS)

## What This Provides
- **Go Toolchain**: Go 1.24 compiler and runtime
- **Development Tools**: air (live reload), delve (debugger), gopls (language server)
- **Code Quality**: golangci-lint, revive, gofmt, goimports
- **Testing**: gotestfmt for enhanced test output
- **Documentation**: gomarkdoc for generating markdown from Go code
- **Formatting**: gofumpt for stricter Go formatting

## Usage
```bash
# Create new project from template
nix flake init -t github:conneroisu/dotfiles#go-shell

# Enter development shell
nix develop

# Start live reload development
air

# Run tests with formatting
go test ./... | gotestfmt

# Format code
nix fmt
```

## Development Workflow
- Use air for automatic recompilation during development
- golangci-lint provides comprehensive linting
- gopls enables rich IDE integration
- All tools configured for optimal Go development experience
*/
{
  description = "A development shell for go";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    treefmt-nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (final: prev: {
            # Add your overlays here
            # Example:
            # my-overlay = final: prev: {
            #   my-package = prev.callPackage ./my-package { };
            # };
            final.buildGoModule = prev.buildGo124Module;
          })
        ];
      };

      rooted = exec:
        builtins.concatStringsSep "\n"
        [
          ''REPO_ROOT="$(git rev-parse --show-toplevel)"''
          exec
        ];

      scripts = {
        dx = {
          exec = rooted ''$EDITOR "$REPO_ROOT"/flake.nix'';
          description = "Edit flake.nix";
        };
        gx = {
          exec = rooted ''$EDITOR "$REPO_ROOT"/go.mod'';
          description = "Edit go.mod";
        };
      };

      scriptPackages =
        pkgs.lib.mapAttrs
        (
          name: script:
            pkgs.writeShellApplication {
              inherit name;
              text = script.exec;
              runtimeInputs = script.deps or [];
            }
        )
        scripts;

      treefmtModule = {
        projectRootFile = "flake.nix";
        programs = {
          alejandra.enable = true; # Nix formatter
        };
      };
    in {
      devShells.default = pkgs.mkShell {
        name = "dev";

        # Available packages on https://search.nixos.org/packages
        packages = with pkgs;
          [
            alejandra # Nix
            nixd
            statix
            deadnix

            go_1_24 # Go Tools
            air
            golangci-lint
            gopls
            revive
            golines
            golangci-lint-langserver
            gomarkdoc
            gotests
            gotools
            reftools
            pprof
            graphviz
            goreleaser
            cobra-cli
          ]
          ++ builtins.attrValues scriptPackages;
      };

      packages = {
        default = pkgs.buildGoModule {
          pname = "my-go-project";
          version = "0.0.1";
          src = self;
          vendorHash = null;
          meta = with pkgs.lib; {
            description = "My Go project";
            homepage = "https://github.com/conneroisu/my-go-project";
            license = licenses.asl20;
            maintainers = with maintainers; [connerohnesorge];
          };
        };
      };

      formatter = treefmt-nix.lib.mkWrapper pkgs treefmtModule;
    });
}
