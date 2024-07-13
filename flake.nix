{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        devShells.default =
          pkgs.mkShell {
            # No way to have buildinputs here :(
            packages = builtins.attrValues {
              inherit (pkgs) cargo rustc rustfmt pre-commit gcc pkg-config bemenu;
              inherit (pkgs.rustPackages) clippy;
            };
            buildInputs = builtins.attrValues {
              inherit (pkgs) gcc pkg-config glib gtk4 gdk-pixbuf cairo gtkd;
            };
          };
      };
      flake = { };
    };
}
