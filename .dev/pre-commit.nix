{ pkgs, config, ... }:
{
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
  settings = {
    hooks = {
      treefmt = {
        enable = true;
      };
      deadnix.enable = true;
      statix = {
        enable = true;
        settings = {
          ignore = [ ".direnv/" ];
          format = "stderr";
        };
      };
      clippy = {
        enable = true;
        settings.offline = false;
      };
    };
  };
}
