{ pkgs, config, ... }:
rec {
  default.packages = builtins.attrValues {
    inherit (pkgs)
      cargo
      rustc
      rustfmt
      gcc
      pkg-config
      bemenu
      wayland
      libxkbcommon
      ;
    inherit (pkgs.rustPackages) clippy;

    inherit (config.pre-commit.settings) package;
  };

  default.env = [
    {
      name = "LD_LIBRARY_PATH";
      value = builtins.foldl' (
        a: b: "${a}:${b}/lib"
      ) "${pkgs.vulkan-loader}/lib" default.packages;
    }
  ];
}
