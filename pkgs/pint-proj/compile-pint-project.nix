{ pkgs, pint }:
pkgs.writeShellApplication {
  name = "pint-proj";

  runtimeInputs = [ pint ];

  text = (builtins.readFile ./build.sh);

}
