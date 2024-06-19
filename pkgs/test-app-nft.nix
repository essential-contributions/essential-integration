{ lib
, stdenv
, darwin
, openssl
, pkg-config
, rustPlatform
}:
let
  src = ../.;
  crateDir = "${src}/apps/nft/front-end";
  crateTOML = "${crateDir}/Cargo.toml";
  lockFile = "${src}/Cargo.lock";
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "nft-front-end";
  version = (builtins.fromTOML (builtins.readFile crateTOML)).package.version;

  OPENSSL_NO_VENDOR = 1;

  nativeBuildInputs = [
    pkg-config
    rust-fmt
    clippy
  ];

  buildInputs = [
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
