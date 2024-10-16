# All essential tools within a single package.
{ essential-rest-client
, essential-node
, essential-builder
, essential-wallet
, essential-debugger
, pint
, symlinkJoin
}:
symlinkJoin {
  name = "essential-all";
  paths = [
    essential-rest-client
    essential-node
    essential-builder
    essential-dry-run
    essential-wallet
    essential-debugger
    pint
  ];
}
