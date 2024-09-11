{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    yafas = {
      url = "github:UbiqueLambda/yafas";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      yafas,
      fenix,
      ...
    }: yafas.allSystems nixpkgs ({ pkgs, system }: {
      devShells.default = pkgs.mkShell.override {
        stdenv = if pkgs.stdenv.isLinux then
          pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv
        else
          pkgs.clangStdenv;
      } rec {
        packages = with pkgs; [
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
            "rust-src"
            "rust-analyzer"
          ])
          gdb
          lldb
          cmake
          pkg-config
        ];
        buildInputs = with pkgs; [
          libGL
          glslang
          vulkan-tools
          vulkan-headers
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          xorg.libxcb
          libxkbcommon
          wayland
          udev
          openssl
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        RUSTFLAGS = "-Zthreads=8";
      };
    }
  );
}
