{
  description = "Conclaude Flake";

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
            bun install --frozen-lockfile
            bun run typecheck
            oxlint --fix
            biome lint --fix
            cd -
          '';
          deps = [pkgs.bun pkgs.oxlint pkgs.biome];
          description = "Lint the project using bun";
        };
        tests = {
          exec = rooted ''
            cd "$REPO_ROOT"
            bun install --frozen-lockfile
            bun test
            cd -
          '';
          deps = [
            pkgs.bun
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

      packages = let
        nodeModules = pkgs.stdenv.mkDerivation {
          pname = "conclaude-node-modules";
          inherit (with builtins; fromJSON (readFile ./package.json)) version;
          src = self;

          nativeBuildInputs = with pkgs; [bun];

          buildPhase = ''
            export HOME=$(mktemp -d)
            export BUN_INSTALL_CACHE_DIR=$(mktemp -d)
            bun install --frozen-lockfile --no-verify
          '';

          installPhase = ''
            mkdir -p $out
            cp -r node_modules $out/
          '';

          outputHashMode = "recursive";
          outputHashAlgo = "sha256";
          outputHash = "sha256-FMGbP7V66qRXBkxdMEYmOFWbL3U6Z0/zEBY8CaP/fb4=";
        };
      in {
        conclaude = pkgs.stdenv.mkDerivation {
          pname = "conclaude";
          inherit (with builtins; fromJSON (readFile ./package.json)) version;
          src = self;

          nativeBuildInputs = with pkgs; [
            bun
            nodejs
          ];

          buildPhase = ''
            runHook preBuild

            # Copy pre-built node_modules
            cp -r ${nodeModules}/node_modules ./
            chmod -R u+w node_modules

            # Build the project
            bun build src/index.ts --target=node --outfile=dist/conclaude.js
            chmod +x dist/conclaude.js

            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall

            mkdir -p $out/bin
            cp dist/conclaude.js $out/bin/conclaude

            runHook postInstall
          '';

          meta = with pkgs.lib; {
            description = "Claude Code hook handler CLI tool that processes hook events and manages lifecycle hooks.";
            homepage = "https://github.com/connix-io/conclaude";
            license = licenses.mit;
            maintainers = with maintainers; [connerohnsorge];
            platforms = platforms.all;
          };
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
