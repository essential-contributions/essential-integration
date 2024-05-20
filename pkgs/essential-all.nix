# All essential tools within a single package.
{ essential-cli
, essential-rest-server
, pintWithSolver
, server-with-rqlite
, symlinkJoin
}:
symlinkJoin {
  name = "essential";
  paths = [
    essential-cli
    essential-rest-server
    pintWithSolver
    server-with-rqlite
  ];
}
