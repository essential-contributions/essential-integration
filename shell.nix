# A dev shell providing the essentials for writing pint apps.
{ essential
, mkShell
}:
mkShell {
  buildInputs = [ essential ];
}
