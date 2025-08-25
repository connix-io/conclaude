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

      packages = {
        conclaude = pkgs.buildNpmPackage {
          pname = "conclaude";
          inherit
            (builtins.fromJSON (builtins.readFile ./package.json))
            version
            ;
          src = self;
          npmDepsHash = "sha256-+LyhSCJHfStHpMlLRTrAggYrxwtnS66mjEvqkXfiAMI=";
          makeCacheWritable = true;
          npmFlags = [ "--legacy-peer-deps" ];
          nativeBuildInputs = [pkgs.bun];
          buildPhase = ''
            bun install --frozen-lockfile
            bun run build
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp dist/conclaude.js $out/bin/conclaude
            chmod +x $out/bin/conclaude
          '';
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
