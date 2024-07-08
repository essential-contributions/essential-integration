# Nix
Nix is the easiest way to get everything you need to start developing with `essential`.
<details>
<summary><b>Install Nix</b></summary>

If you don't already have Nix installed you can install it by running the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```
This uses the [Determinate Systems installer](https://determinate.systems/posts/determinate-nix-installer/). \
There are other alternatives [here.](https://nixos.org/download/) \
You can lean more about Nix [here.](https://nixos.org/)
</details>

## Enter development shell
This will enter you into a shell with `cargo`, `pint`, `essential-server` and some other things that will be useful for developing your application.
```bash
nix develop github:essential-contributions/essential-integration
```
