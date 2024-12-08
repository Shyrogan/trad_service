{ pkgs, ... }: {
  packages = with pkgs; [ nushell ];
  enterShell = ''
    nu
  '';
  languages.rust = {
    enable = true;
  };
  services = {
    redis.enable = true;
    postgres = {
      enable = true;
      initialDatabases = [{
        name = "trad";
        user = "dev";
        pass = "dev";
      }];
    };
  };
  git-hooks.hooks = {
    clippy.enable = true;
  };
  env = {
    ADDRESS = "0.0.0.0:3000";
    PG_URL = "postgresql://dev:dev@localhost/trad?application_name=trad_service";
  };
}
