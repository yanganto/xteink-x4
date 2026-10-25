{
  description = "Xteink X4 ESP32-C3 firmware";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    parachutes = {
      url = "github:yanganto/parachutes";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, parachutes }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            parachutes.overlays.myMkShell
          ];
        };
        # Reads channel and targets directly from rust-toolchain.toml
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      let
        buildScript = pkgs.writeShellScriptBin "build"
          (builtins.readFile ./nix/scripts/build.sh);
        deployScript = pkgs.writeShellScriptBin "deploy"
          (builtins.readFile ./nix/scripts/deploy.sh);
      in
      {
        devShells.default = pkgs.myMkShell {
          name = "default";
          nativeBuildInputs = [
            toolchain
            pkgs.espflash
            pkgs.pkg-config
            pkgs.tio
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            pkgs.udev
          ];

          myScripts = [
            buildScript
            deployScript
          ];
        };
      });
}
