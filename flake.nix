{
  description = "Conclaude Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    bun2nix.url = "github:baileyluTCD/bun2nix";
    bun2nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    bun2nix,
    treefmt-nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (final: prev: {})
        ];
      };

      rooted = exec:
        builtins.concatStringsSep "\n"
        [
          ''
            REPO_ROOT=$(git rev-parse --show-toplevel)
            export REPO_ROOT
          ''
          exec
        ];

      scripts = {
        dx = {
          exec = rooted ''
            $EDITOR "$REPO_ROOT"/flake.nix
          '';
          deps = [pkgs.git];
          description = "Edit flake.nix";
        };
        lint = {
          exec = rooted ''
            cd "$REPO_ROOT"
            bun run typecheck
            oxlint --fix
            biome lint --fix
            cd -
          '';
          deps = [pkgs.bun pkgs.oxlint pkgs.biome];
          description = "Lint the project using bun";
        };
        tests = {
          exec = ''
            bun test
          '';
          deps = [
            pkgs.pkg-config
            pkgs.openssl
            pkgs.openssl.dev
          ];
          description = "Run tests with bun";
        };
      };

      scriptPackages =
        pkgs.lib.mapAttrs
        (
          name: script: let
            scriptType = script.type or "app";
          in
            if script != {}
            then
              if scriptType == "script"
              then pkgs.writeShellScriptBin name script.exec
              else
                pkgs.writeShellApplication {
                  inherit name;
                  bashOptions = scripts.baseOptions or ["errexit" "pipefail" "nounset"];
                  text = script.exec;
                  runtimeInputs = script.deps or [];
                }
            else null
        )
        scripts;
    in {
      devShells = {
        default = pkgs.mkShell {
          env = {
            NIX_CONFIG = "cores = 4\nmax-jobs = 4";
            NODE_OPTIONS = "--max-old-space-size=4096";
          };
          packages =
            (with pkgs; [
              alejandra # Nix
              nixd
              statix
              deadnix
              just
              sqlite
              eslint
              oxlint
              bun
              bun2nix.packages.${system}.default
              openssl
              openssl.dev
              pkg-config
              biome
              typescript-language-server
              vscode-langservers-extracted
              tailwindcss-language-server
              yaml-language-server
            ])
            ++ builtins.attrValues scriptPackages;
          shellHook = ''
            echo "🔥 Conclaude Development Environment"
          '';
        };
      };

      apps =
        pkgs.lib.mapAttrs
        (name: script: {
          type = "app";
          program = "${scriptPackages.${name}}/bin/${name}";
        })
        (pkgs.lib.filterAttrs (_: script: script != {}) scripts);

      packages = {
        conclaude = bun2nix.lib.${system}.mkBunDerivation {
          pname = "conclaude";
          src = self;
          inherit
            (builtins.fromJSON (builtins.readFile ./package.json))
            version
            ;
          bunNix = ./bun.nix;
          index = "./src/index.ts";
        };
        default = self.packages.${system}.conclaude;
      };

      formatter = let
        treefmtModule = {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true; # Nix formatter
            biome.enable = true; ### TypeScript formatter
          };
          settings.formatter.biome = {
            command = "${pkgs.biome}/bin/biome";
            includes = ["*.ts" "*.tsx"];
          };
        };
      in
        treefmt-nix.lib.mkWrapper pkgs treefmtModule;
    });
}
