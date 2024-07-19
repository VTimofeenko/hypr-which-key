{ pkgs, ... }:
{
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
        extraPackages = [ pkgs.gcc ];
      };
    };
  };
}
