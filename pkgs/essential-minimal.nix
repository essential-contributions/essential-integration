# The minimal essential tools for app dev.
{ essential-builder
, essential-node
, pint
, symlinkJoin
}:
symlinkJoin {
  name = "essential-minimal";
  paths = [
    essential-builder
    essential-node
    pint
  ];
}
