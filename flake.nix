{
  description = ''
    Integration between the Essential protocol and the Pint language.
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    # The essential server.
    essential-server = {
      url = "git+ssh://git@github.com/essential-contributions/essential-server";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # The pint programming language.
    pint = {
      url = "git+ssh://git@github.com/essential-contributions/pint.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    cargo-readme-src = {
      url = "github:webern/cargo-readme";
      flake = false;
    };
  };

  outputs = inputs:
    let
      overlays = [
        inputs.essential-server.overlays.default
        inputs.pint.overlays.default
        inputs.self.overlays.default
      ];

      # Functions for accessing pkgs per system.
      perSystemPkgs = withPkgs:
        inputs.nixpkgs.lib.genAttrs (import inputs.systems)
          (system: withPkgs (import inputs.nixpkgs { inherit overlays system; }));
    in
    {
      overlays = {
        essential-integration = final: prev: {
          # CLI utilities.
          essential-cli = prev.callPackage ./pkgs/essential-cli.nix { };
          # Essential REST client.
          essential-rest-client = prev.callPackage ./pkgs/essential-rest-client.nix { };
          # Essential deploy contract.
          essential-deploy-contract = prev.callPackage ./pkgs/essential-deploy-contract.nix { };
          # All essential applications under one package.
          essential = final.callPackage ./pkgs/essential-all.nix { };
          # Build cargo readme.
          cargo-readme = final.callPackage ./pkgs/cargo-readme.nix { inherit (inputs) cargo-readme-src; };
          # All app tests.
          test-app-counter = final.callPackage ./pkgs/test-app-counter.nix { };
        };
        default = inputs.self.overlays.essential-integration;
      };

      packages = perSystemPkgs (pkgs: {
        essential-cli = pkgs.essential-cli;
        essential-rest-client = pkgs.essential-rest-client;
        essential-deploy-contract = pkgs.essential-deploy-contract;
        essential = pkgs.essential;
        test-app-counter = pkgs.test-app-counter;
        cargo-readme = pkgs.cargo-readme;
        default = inputs.self.packages.${pkgs.system}.essential;
      });

      devShells = perSystemPkgs (pkgs: {
        essential-integration-dev = pkgs.callPackage ./shell.nix { };
        default = inputs.self.devShells.${pkgs.system}.essential-integration-dev;
      });

      formatter = perSystemPkgs (pkgs: pkgs.nixpkgs-fmt);
    };
}
