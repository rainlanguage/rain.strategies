{
  inputs = {
    rainix.url = "github:rainlanguage/rainix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { flake-utils, rainix, ... }:
    flake-utils.lib.eachDefaultSystem (system: rec {
      packages = rainix.packages.${system};

      devShells = rainix.devShells.${system};
    });
}
