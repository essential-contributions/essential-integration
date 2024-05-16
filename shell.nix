# A dev shell providing the essentials for writing pint apps.
{ essential-rest-server
, mkShell
, pint
, server-with-rqlite
}:
mkShell {
  buildInputs = [
    essential-rest-server
    pint
    server-with-rqlite
  ];
}
