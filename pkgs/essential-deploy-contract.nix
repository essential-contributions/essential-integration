{ lib
, stdenv
, darwin
, openssl
, pkg-config
, rustPlatform
}:
let
  src = builtins.path {
    path = ../.;
    filter = path: type:
      let
        keepFiles = [
          "Cargo.lock"
          "Cargo.toml"
          "crates"
          "apps"
        ];
        includeDirs = [
          "crates"
          "apps"
        ];
        isPathInIncludeDirs = dir: lib.strings.hasInfix dir path;
      in
      if lib.lists.any (p: p == (baseNameOf path)) keepFiles then
        true
      else
        lib.lists.any (dir: isPathInIncludeDirs dir) includeDirs
    ;
  };
  crateDir = "${src}/crates/essential-deploy-contract";
  crateTOML = "${crateDir}/Cargo.toml";
  lockFile = "${src}/Cargo.lock";
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "essential-deploy-contract";
  version = (builtins.fromTOML (builtins.readFile crateTOML)).package.version;

  buildAndTestSubdir = "crates/essential-deploy-contract";

  OPENSSL_NO_VENDOR = 1;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    openssl
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  cargoLock = {
    inherit lockFile;
  };

  doCheck = false;
}
