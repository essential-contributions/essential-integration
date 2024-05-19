# essential-integration

Integration of the Pint constraint language and the Essential protocol.

## Using Nix

A Nix flake is included providing the `essential` package, providing `pintc`,
`pintfmt`, `essential-rest-server` and more.

1. Install Nix, easiest with the [Determinate Systems installer](https://github.com/DeterminateSystems/nix-installer).

2. Use Nix to enter a shell with the all Essential tools:
   ```console
   nix shell git+ssh://git@github.com/essential-contributions/essential-integration
   ```
