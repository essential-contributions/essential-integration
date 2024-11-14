{
  description = ''
    Integration between the Essential protocol and the Pint language.
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    # The essential node.
    essential-node = {
      url = "github:essential-contributions/essential-node";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "nixpkgs";
    };

    # The essential builder.
    essential-builder = {
      url = "github:essential-contributions/essential-builder";
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
        inputs.essential-node.overlays.default
        inputs.essential-builder.overlays.default
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
          # Essential REST client.
          essential-rest-client = prev.callPackage ./pkgs/essential-rest-client.nix { };
          # All essential applications under one package.
          essential = final.callPackage ./pkgs/essential.nix { };
          # Build cargo readme.
          cargo-readme = final.callPackage ./pkgs/cargo-readme.nix { inherit (inputs) cargo-readme-src; };
          # The book.
          book = final.callPackage ./pkgs/book.nix { };
          # Pint project compiler.
          pint-proj = prev.callPackage ./pkgs/pint-proj/compile-pint-project.nix { };
          # Compile all pint projects in this repo under apps
          compile-all-contracts = final.callPackage ./pkgs/pint-proj/compile-all-apps.nix { };
        };
        default = inputs.self.overlays.essential-integration;
      };

      packages = perSystemPkgs (pkgs: {
        essential-builder = pkgs.essential-builder;
        essential-node = pkgs.essential-node;
        essential-rest-client = pkgs.essential-rest-client;
        essential = pkgs.essential;
        book = pkgs.book;
        cargo-readme = pkgs.cargo-readme;
        pint = pkgs.pint;
        pint-proj = pkgs.pint-proj;
        compile-all-contracts = pkgs.compile-all-contracts;
        default = inputs.self.packages.${pkgs.system}.essential;
      });

      devShells = perSystemPkgs (pkgs: {
        dev = pkgs.callPackage ./shells/shell.nix { };
        default = inputs.self.devShells.${pkgs.system}.dev;
      });

      templates = {
        pint-app = {
          path = ./templates;
          description = "A Pint project.";
          welcomeText = ''
            # Pint Project

            This is a Pint project. It is a template for creating a new Pint project.

            ## Getting Started

            To get started, run the following command:

            ```shell
            nix develop
            ```
          '';
        };

        default = inputs.self.templates.pint-app;
      };

      formatter = perSystemPkgs (pkgs: pkgs.nixpkgs-fmt);
    };
}
