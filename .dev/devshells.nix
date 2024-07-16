{ pkgs, config, ... }:
{
  default.packages = builtins.attrValues {
    inherit (pkgs)
      cargo
      rustc
      rustfmt
      gcc
      pkg-config
      bemenu
      ;
    inherit (pkgs.rustPackages) clippy;

    inherit (config.pre-commit.settings) package;
  };
}
