_: {
  # Override hte environment for checking the flake in pure environment
  # Source: https://github.com/cachix/git-hooks.nix/issues/452#issuecomment-2163017537
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
