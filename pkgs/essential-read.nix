{ rustPlatform
}:
let
  src = ../.;
  crateDir = "${src}/crates/essential-read";
  crateTOML = "${crateDir}/Cargo.toml";
  lockFile = "${src}/Cargo.lock";
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "essential-read";
  version = (builtins.fromTOML (builtins.readFile crateTOML)).package.version;

  nativeBuildInputs = [
    pkg-config
  ];

  cargoLock = {
    inherit lockFile;
  };

  doCheck = false
    }
