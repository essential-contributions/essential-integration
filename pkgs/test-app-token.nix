# This package builds and tests the token app.
{ lib
, stdenv
, darwin
, openssl
, pkg-config
, rustPlatform
}:
let
  src = ../.;
  crateDir = "${src}/apps/token/app";
  crateTOML = "${crateDir}/Cargo.toml";
  lockFile = "${src}/Cargo.lock";
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "app";
  version = (builtins.fromTOML (builtins.readFile crateTOML)).package.version;

  nativeBuildInputs = lib.optionals stdenv.isLinux [
    pkg-config
  ];

  buildInputs = lib.optionals stdenv.isLinux [
    openssl
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  cargoLock = {
    inherit lockFile;
    # FIXME: This enables using `builtins.fetchGit` which uses the user's local
    # `git` (and hence ssh-agent for ssh support). Once the repos are public,
    # this should be removed.
    allowBuiltinFetchGit = true;
  };
}
