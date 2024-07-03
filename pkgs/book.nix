{ stdenv
, mdbook
}:
stdenv.mkDerivation {
  pname = "getting-started";
  version = "0.1";
  src = ./../docs;
  buildInputs = [ mdbook ];
  phases = [ "unpackPhase" "buildPhase" ];
  buildPhase = ''
    mkdir -p $out
    mdbook build -d $out getting-started
  '';
}
