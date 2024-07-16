_: {
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
    };
  };
}
