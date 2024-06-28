# A dev shell providing the essentials for writing pint apps.
{ essential
, jq
, mkShell
, xxd
, darwin
, cargo-toml-lint
, cargo-readme
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
    cargo-readme
    essential
    jq
    xxd
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
