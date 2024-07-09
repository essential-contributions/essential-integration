{
  description = ''
    Integration between the Essential protocol and the Pint language.
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

    # The essential debugger.
    essential-debugger = {
      url = "github:essential-contributions/essential-debugger";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # The pint programming language.
    pint = {
      url = "github:essential-contributions/pint.nix";
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
        inputs.essential-wallet.overlays.default
        inputs.essential-debugger.overlays.default
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
          # Essential dry run.
          essential-dry-run = prev.callPackage ./pkgs/essential-dry-run.nix { };
          # All essential applications under one package.
          essential-all = final.callPackage ./pkgs/essential-all.nix { };
          # The minimal essential applications under one package.
          essential-minimal = final.callPackage ./pkgs/essential-minimal.nix { };
          # Build cargo readme.
          cargo-readme = final.callPackage ./pkgs/cargo-readme.nix { inherit (inputs) cargo-readme-src; };
          # All app tests.
          test-app-counter = final.callPackage ./pkgs/test-app-counter.nix { };
          # The book.
          book = final.callPackage ./pkgs/book.nix { };
          # Pint project compiler.
          pint-proj = prev.callPackage ./pkgs/compile-pint-project.nix { };
        };
        default = inputs.self.overlays.essential-integration;
      };

      packages = perSystemPkgs (pkgs: {
        essential-cli = pkgs.essential-cli;
        essential-rest-client = pkgs.essential-rest-client;
        essential-deploy-contract = pkgs.essential-deploy-contract;
        essential-dry-run = pkgs.essential-dry-run;
        essential-all = pkgs.essential-all;
        essential-minimal = pkgs.essential-minimal;
        test-app-counter = pkgs.test-app-counter;
        book = pkgs.book;
        cargo-readme = pkgs.cargo-readme;
        pint-proj = pkgs.pint-proj;
        default = inputs.self.packages.${pkgs.system}.essential-minimal;
      });

      devShells = perSystemPkgs (pkgs: {
        dev = pkgs.callPackage ./shells/dev.nix { };
        essential-rust-app-dev = pkgs.callPackage ./shells/essential-rust-app-dev.nix { };
        essential-integration-dev = pkgs.callPackage ./shells/shell.nix { };
        default = inputs.self.devShells.${pkgs.system}.essential-integration-dev;
      });

      formatter = perSystemPkgs (pkgs: pkgs.nixpkgs-fmt);
    };
}
