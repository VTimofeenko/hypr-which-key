{ self, ... }:
{
  lib,
  config,
  pkgs,
  ...
}:
let
  cfg = config.services.hypr-which-key;
  inherit (lib) mkEnableOption mkIf mkPackageOption;
in
{
  # Interface
  options.services.hyprland-language-switch-notifier = {
    enable = mkEnableOption "hyprland language switch notifications";
    package = mkPackageOption self.packages.${pkgs.system} "hypr-which-key" { };
  };

  # Impl
  config = mkIf cfg.enable {
    systemd.user.services.hypr-which-key = {
      Unit = {
        Description = "Service that shows a popup with key bindings";
        BindsTo = [ "hyprland-session.target" ];
      };
      Service = {
        ExecStart = "${lib.getExe cfg.package}";
      };
      Install.WantedBy = [ "graphical-session.target" ];
    };
  };
}
