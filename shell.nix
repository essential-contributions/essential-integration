# A dev shell providing the essentials for writing pint apps.
{ essential
, jq
, mkShell
, xxd
}:
mkShell {
  buildInputs = [
    essential
    jq
    xxd
  ];
}
