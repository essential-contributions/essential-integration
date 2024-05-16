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
  };

  outputs = inputs: let
    overlays = [
      inputs.essential-server.overlays.default
      inputs.pint.overlays.default
      inputs.self.overlays.default
    ];

    # Functions for accessing pkgs per system.
    perSystemPkgs = withPkgs:
      inputs.nixpkgs.lib.genAttrs (import inputs.systems)
        (system: withPkgs (import inputs.nixpkgs { inherit overlays system; }));
  in {
    overlays = {
      essential-integration = final: prev: {
        # Declare packages here.
      };
      default = inputs.self.overlays.essential-integration;
    };

    packages = perSystemPkgs (pkgs: {
      # Export overlay packages here.
    });

    devShells = perSystemPkgs (pkgs: {
      essential-integration-dev = pkgs.callPackage ./shell.nix { };
      default = inputs.self.devShells.${pkgs.system}.essential-integration-dev;
    });

    formatter = perSystemPkgs (pkgs: pkgs.nixpkgs-fmt);
  };
}
