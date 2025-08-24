{
  pkgs,
  mkShell,
  lib,
  ...
}:
mkShell rec {
  packages = with pkgs; [
    cmake
    pkg-config
    python3
    ninja
    fontconfig
    freetype

    rustc
    rustfmt
    cargo
    clippy
    rust-analyzer
    libxkbcommon
    wayland

    xorg.libX11
    xorg.libXrandr
    xorg.libXi
    xorg.libXcursor
    xorg.libXinerama
    xorg.libXxf86vm
    xorg.libXrender
    xorg.libxcb
    mesa
    mesa.drivers   # <- drivers EGL/DRI
    libglvnd       # <- despacha GL/EGL para o driver certo

    wayland
    wayland-protocols
    libdrm
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath packages;
}
