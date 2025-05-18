{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    ,
    }:
    flake-utils.lib.eachDefaultSystem
      (
        system:
        let
          overlays = [
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnfree = true;
          };
          rustToolchain = with pkgs; [ rust-bin.stable.latest.default ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            git
          ];

          buildInputs = with pkgs; [
            cmake
            udev
            ninja
            openssl
            pango
            gdk-pixbuf
            gtk3
            libdrm
            mesa
            wlroots
            libopus
            alsa-lib
            vulkan-loader
            systemd
            libxkbcommon
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
            xorg.xkbcomp
            xorg.xkbutils
            xorg.xkbevd
            libGL
            wayland
          ];
        in
        with pkgs; {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
            shellHook = ''
              export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}
            '';
          };
        }
      );
}

