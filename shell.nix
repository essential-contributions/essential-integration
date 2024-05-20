# A dev shell providing the essentials for writing pint apps.
{ essential
, jq
, mkShell
}:
mkShell {
  buildInputs = [
    essential
    jq
  ];
}
