{
  perSystem = let
    crateName = "nixpkgs-news";
  in {
    nci.projects.${crateName}.path = ./.;
    nci.crates.${crateName} = {
      env = {inherit FLAKE_PROVIDED_NIXPKGS_CLONE_PATH;};
    };
  };
}
