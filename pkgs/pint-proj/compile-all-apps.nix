{ pkgs, pint-proj }:
pkgs.writeShellApplication {
  name = "compile-all-apps";

  runtimeInputs = [ pint-proj ];

  text = ''
    pint-proj -r apps
  '';

}
