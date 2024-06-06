# All essential tools within a single package.
{ essential-cli
, essential-rest-client
, essential-rest-server
, pint
, server-with-rqlite
, symlinkJoin
}:
symlinkJoin {
  name = "essential";
  paths = [
    essential-cli
    essential-rest-client
    essential-rest-server
    pint
    server-with-rqlite
  ];
}
