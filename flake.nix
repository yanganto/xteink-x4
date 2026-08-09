{
  description = "Xteink X4 ESP32-C3 firmware";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        # Reads channel and targets directly from rust-toolchain.toml
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      let
        buildScript = pkgs.writeShellScriptBin "build" ''
          cargo build --release
        '';
        deployScript = pkgs.writeShellScriptBin "deploy" ''
          espflash flash --chip esp32c3 target/riscv32imc-unknown-none-elf/release/xteink-x4
        '';
        deployAndMonitorScript = pkgs.writeShellScriptBin "deploy_and_monitor" ''
          espflash flash --chip esp32c3 --monitor target/riscv32imc-unknown-none-elf/release/xteink-x4
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
            pkgs.espflash
            pkgs.pkg-config
            pkgs.tio
            buildScript
            deployScript
            deployAndMonitorScript
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            pkgs.udev
          ];
        };
      });
}
