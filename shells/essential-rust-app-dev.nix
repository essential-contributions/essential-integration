# Shell for building pint apps with rust front ends
{ essential-minimal
, mkShell
, darwin
, cargo-toml-lint
, clippy
, rqlite
, rust-analyzer
, rustfmt
, rustc
, cargo
, lib
, stdenv
, libiconv
, openssl
, sqlite
}:
mkShell {
  buildInputs = [
    essential-minimal
    libiconv
    cargo-toml-lint
    clippy
    rqlite
    rust-analyzer
    rustfmt
    openssl
    openssl.dev
    rustc
    cargo
    sqlite
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
