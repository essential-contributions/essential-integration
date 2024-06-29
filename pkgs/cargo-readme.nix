{ lib
, stdenv
, rustPlatform
, cargo-readme-src
}:
let
  src = cargo-readme-src;
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "cargo-readme";
  version = "3.3.1";

  doCheck = false;

  cargoSha256 = "sha256-OEArMqOiT+PZ+zMRt9h0EzeP7ikFuOYR8mFGtm+xCkQ=";

  meta = with lib; {
    description = "A cargo subcommand to generate README.md content from doc comments";
    mainProgram = "cargo-readme";
    homepage = "https://github.com/webern/cargo-readme";
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ webern ];
  };
}
