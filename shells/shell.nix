# A dev shell providing the essentials for writing pint apps.
{ essential-minimal
, essential-cli
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
, mdbook
, rustc
, cargo
}:
mkShell {
  buildInputs = [
    cargo-readme
    essential-minimal
    essential-cli
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
    mdbook
    rustc
    cargo
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
