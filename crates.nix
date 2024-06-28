{
  perSystem = let
    crateName = "nixpkgs-news";
  in {
    nci.projects.${crateName}.path = ./.;
    nci.crates.${crateName} = {};
  };
}
