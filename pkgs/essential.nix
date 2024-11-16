# All essential tools within a single package.
{ essential-rest-client
, essential-node
, essential-builder
, essential-wallet
, essential-wallet-test
, essential-debugger
, pint
, pint-deploy
, pint-submit
, symlinkJoin
}:
symlinkJoin {
  name = "essential";
  paths = [
    essential-rest-client
    essential-node
    essential-builder
    essential-wallet
    essential-wallet-test
    essential-debugger
    pint
    pint-deploy
    pint-submit
  ];
}
