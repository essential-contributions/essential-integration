{ rustPlatform
}:
let
  src = ../.;
  crateDir = "${src}/crates/essential-read";
  crateTOML = "${crateDir}/Cargo.toml";
  lockFile = "${src}/Cargo.lock";
in
rustPlatform.buildRustPackage {
  inherit src;
  pname = "essential-read";
  version = (builtins.fromTOML (builtins.readFile crateTOML)).package.version;

  nativeBuildInputs = [
    pkg-config
  ];

  cargoLock = {
    inherit lockFile;
    # FIXME: This enables using `builtins.fetchGit` which uses the user's local
    # `git` (and hence ssh-agent for ssh support). Once the repos are public,
    # this should be removed.
    allowBuiltinFetchGit = true;
  };

  doCheck = false
    }
