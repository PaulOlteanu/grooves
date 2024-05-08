{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages = {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      };

      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            # https://devenv.sh/reference/options/
            dotenv.enable = true;
            packages = [pkgs.flyctl];

            # languages.rust = {
            #   enable = true;
            #   channel = "stable";
            #   mold.enable = false;
            #   components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src"];
            # };

            services.postgres = {
              enable = true;
              listen_addresses = "127.0.0.1";
              initialDatabases = [
                {name = "grooves-dev";}
              ];

              initialScript = ''
                CREATE USER grooves SUPERUSER;
                ALTER USER grooves WITH PASSWORD 'groovesdevpassword';
              '';
            };
          }
        ];
      };
    });
}
