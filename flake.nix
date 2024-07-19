{
  description = "Which-key for hyprland";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, self, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { flake-parts-lib, ... }:
      {
        imports = [
          inputs.devshell.flakeModule
          inputs.pre-commit-hooks-nix.flakeModule
          inputs.treefmt-nix.flakeModule
        ];
        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];
        perSystem =
          {
            config,
            # inputs',
            pkgs,
            # system,
            ...
          }:
          {
            packages =
              let
                craneLib = inputs.crane.mkLib pkgs;
                commonArgs = {
                  src = craneLib.cleanCargoSource ./.;
                  strictDeps = true;
                  buildInputs = [
                    pkgs.bemenu # TODO: make this configurable in a module or something
                  ];
                };
              in
              rec {
                default = hypr-which-key;
                hypr-which-key = craneLib.buildPackage (
                  commonArgs // { cargoArtifacts = craneLib.buildDepsOnly commonArgs; }
                );
              };
            # Dev deps
            devshells = import ./.dev/devshells.nix { inherit pkgs config; };
            treefmt = import ./.dev/treefmt.nix { };
            pre-commit = import ./.dev/pre-commit.nix { };
            # Override hte environment for checking the flake in pure environment
            # Source: https://github.com/cachix/git-hooks.nix/issues/452#issuecomment-2163017537
            checks.pre-commit = pkgs.lib.mkForce (
              let
                drv = config.pre-commit.settings.run;
              in
              pkgs.stdenv.mkDerivation {
                name = "pre-commit-run";
                src = config.pre-commit.settings.rootSrc;
                buildInputs = [
                  pkgs.git
                  pkgs.openssl
                  pkgs.pkg-config
                ];
                nativeBuildInputs = [ pkgs.rustPlatform.cargoSetupHook ];
                cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
                buildPhase = drv.buildCommand;
              }
            );
            devShells.pre-commit = config.pre-commit.devShell;
          };
        flake.homeManagerModules.default =
          let
            inherit (flake-parts-lib) importApply;
          in
          importApply ./nix/homeManagerModule.nix { inherit self; };
      }
    );
}
