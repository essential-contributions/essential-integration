# The minimal essential tools for app dev.
{ essential-rest-server
, pint
, symlinkJoin
}:
symlinkJoin {
  name = "essential-minimal";
  paths = [
    essential-rest-server
    pint
  ];
}
