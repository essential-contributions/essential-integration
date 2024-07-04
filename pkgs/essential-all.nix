# All essential tools within a single package.
{ essential-cli
, essential-rest-client
, essential-rest-server
, essential-deploy-contract
, essential-dry-run
, essential-wallet
, essential-debugger
, pint
, server-with-rqlite
, symlinkJoin
}:
symlinkJoin {
  name = "essential-all";
  paths = [
    essential-cli
    essential-rest-client
    essential-rest-server
    essential-deploy-contract
    essential-dry-run
    essential-wallet
    essential-debugger
    pint
    server-with-rqlite
  ];
}
