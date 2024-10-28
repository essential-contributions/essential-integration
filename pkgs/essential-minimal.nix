# The minimal essential tools for app dev.
{ essential-builder
, essential-node
, essential-rest-client
, pint
, symlinkJoin
}:
symlinkJoin {
  name = "essential-minimal";
  paths = [
    essential-rest-client
    essential-builder
    essential-node
    pint
  ];
}
