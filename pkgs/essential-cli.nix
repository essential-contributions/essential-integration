{ rustPlatform
}:
let
  src = ../crates/essential-cli;
  manifest-path = "${src}/Cargo.toml";
  manifest = builtins.fromTOML (builtins.readFile manifest-path);
in
rustPlatform.buildRustPackage {
  pname = manifest.package.name;
  version = manifest.package.version;
  inherit src;

  cargoLock = {
    lockFile = "${src}/Cargo.lock";
    # FIXME: This enables using `builtins.fetchGit` which uses the user's local
    # `git` (and hence ssh-agent for ssh support). Once the repos are public,
    # this should be removed.
    allowBuiltinFetchGit = true;
  };

  doCheck = false;
}
