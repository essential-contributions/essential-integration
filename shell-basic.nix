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
, mdbook
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
    mdbook
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
