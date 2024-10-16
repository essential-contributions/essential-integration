# Shell for building pint apps with al the tools
{ essential-all
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
, rustc
, cargo
, sqlite
}:
mkShell {
  buildInputs = [
    essential-all
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
