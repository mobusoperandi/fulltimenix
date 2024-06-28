{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  inputs.treefmt-nix.url = "github:numtide/treefmt-nix";

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-darwin"];

      imports = [
        nci.flakeModule
        inputs.treefmt-nix.flakeModule
        ./crates.nix
      ];

      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateOutputs = config.nci.outputs.nixpkgs-news;
      in {
        devShells.default = crateOutputs.devShell.overrideAttrs (old: {
          packages = [pkgs.rust-analyzer];
          NIXPKGS_CLONE_PATH = "faux";
        });

        packages.default = crateOutputs.packages.release;

        treefmt.projectRootFile = "flake.nix";
        treefmt.programs.alejandra.enable = true;
        treefmt.programs.rustfmt.enable = true;
      };
    };
}
