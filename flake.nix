{
  description = "A development shell for rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    treefmt-nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (craneLib.filterCargoSources path type)
          || (builtins.match ".*default-config\\.yaml$" path != null);
      };

      conclaude = craneLib.buildPackage {
        inherit src;
        strictDeps = true;
        pname = "conclaude";
        version = "0.1.1";

        buildInputs = with pkgs; [
          # Add any system dependencies here if needed
        ];
      };

      rooted = exec:
        builtins.concatStringsSep "\n"
        [
          ''
            REPO_ROOT="$(git rev-parse --show-toplevel)"
          ''
          exec
        ];
      scripts = {
        dx = {
          exec = rooted ''$EDITOR "$REPO_ROOT"/flake.nix'';
          description = "Edit flake.nix";
        };
        rx = {
          exec = rooted ''$EDITOR "$REPO_ROOT"/Cargo.toml'';
          description = "Edit Cargo.toml";
        };
        tests = {
          exec = rooted ''
            cd "$REPO_ROOT"
            cargo test
            cd -
          '';
          description = "Run tests";
        };
        lint = {
          exec = rooted ''
            cd "$REPO_ROOT"
            cargo clippy --all-targets --all-features
            cd -
          '';
          description = "Run clippy";
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
    in {
      packages.default = conclaude;

      devShells.default = pkgs.mkShell {
        name = "dev";
        # Available packages on https://search.nixos.org/packages
        buildInputs = with pkgs;
          [
            alejandra # Nix
            nixd
            statix
            deadnix
            just
            rust-bin.stable.latest.default
            rust-bin.stable.latest.rust-analyzer
            napi-rs-cli
            yarn
          ]
          ++ builtins.attrValues scriptPackages;
        shellHook = ''
          echo "Welcome to the rust devshell!"
        '';
      };

      formatter = let
        treefmtModule = {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true; # Nix formatter
            rustfmt.enable = true; # Rust formatter
          };
        };
      in
        treefmt-nix.lib.mkWrapper pkgs treefmtModule;
    });
}
