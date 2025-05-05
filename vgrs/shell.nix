let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
in
with pkgs;
mkShell {
  buildInputs = [
    probe-rs-tools # flashing / debugging
    gdb
    picocom
    udev pkg-config
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];
}
