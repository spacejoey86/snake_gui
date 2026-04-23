/*
based on
https://discourse.nixos.org/t/how-can-i-set-up-my-rust-programming-environment/4501/9
*/
let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "1.92.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell rec {
  buildInputs = [
    rust
  ] ++ (with pkgs; [
    pkg-config
    # other dependencies
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libxkbcommon
    xorg.libxcb
    vulkan-loader
    glfw
    libGL
  ]);

  RUST_BACKTRACE = 1;

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
  '';
}
