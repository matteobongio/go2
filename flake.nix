{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
      name = "go2";
      cargoLock = {
        lockFile = ./Cargo.lock;
      };
      src = ./.;
    };
  };
}
