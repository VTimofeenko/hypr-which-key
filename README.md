Implementation of which-key for hyprland's submaps. Shows a menu with this
submap's key mappings:

![](./.assets/screenshot.png)

The menu will pop up whenever you enter a submap and close whenever the submap
is reset.

*NOTE*: this project is very much in alpha stage. See TODOs below.

# Usage

```
nix run github:VTimofeenko/hypr-which-key
```

or import `homeManagerModules.default` from this flake and enable the service:

```nix
services.hypr-which-key.enable = true;
```

# TODO

- [X] Proper pre-commit/format stuff
- [X] Nix package
- [X] Add a sane way to show aliases
- [ ] Try to cover with tests as much as possible
- [X] Nix modules (checks?)
- [ ] A proper UI instead of horrible echo-into-bemenu hack
- [ ] Gif of project
- [ ] Add "show all shortcuts" mode
