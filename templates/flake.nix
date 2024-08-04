{
  description = ''
    Flake for essential application development.
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    # The essential server.
    essential-server = {
      url = "github:essential-contributions/essential-server";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # The essential wallet.
    essential-wallet = {
      url = "github:essential-contributions/essential-wallet";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # Essential integration.
    essential-integration = {
      url = "github:essential-contributions/essential-integration";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # The pint programming language.
    pint = {
      url = "github:essential-contributions/pint.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    let
      overlays = [
        inputs.essential-server.overlays.default
        inputs.essential-wallet.overlays.default
        inputs.essential-integration.overlays.default
        inputs.pint.overlays.default
      ];

      # Functions for accessing pkgs per system.
      perSystemPkgs = withPkgs:
        inputs.nixpkgs.lib.genAttrs (import inputs.systems)
          (system: withPkgs (import inputs.nixpkgs { inherit overlays system; }));
    in
    {

      packages = perSystemPkgs (pkgs: {
        compile-pint-project = pkgs.pint-proj;
        default = inputs.self.packages.${pkgs.system}.compile-pint-project;
      });

      devShells = perSystemPkgs (pkgs: {
        dev = pkgs.mkShell {
          buildInputs = [
            pkgs.essential-rest-server
            pkgs.essential-wallet
            pkgs.essential-deploy-contract
            pkgs.pint
            pkgs.clippy
            pkgs.rqlite
            pkgs.rust-analyzer
            pkgs.rustfmt
            pkgs.openssl
            pkgs.openssl.dev
            pkgs.rustc
            pkgs.cargo
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            pkgs.libiconv
          ];
        };
        default = inputs.self.devShells.${pkgs.system}.dev;
      });

      formatter = perSystemPkgs (pkgs: pkgs.nixpkgs-fmt);
    };
}
