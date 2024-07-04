# Shell for building pint apps with rust front ends
{ essential-rest-server
, mkShell
, darwin
, cargo-toml-lint
, clippy
, rqlite
, rust-analyzer
, rustfmt
, lib
, stdenv
, libiconv
, openssl
}:
mkShell {
  buildInputs = [
    essential-rest-server
    libiconv
    cargo-toml-lint
    clippy
    rqlite
    rust-analyzer
    rustfmt
    openssl
    openssl.dev
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
