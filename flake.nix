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
      devShells.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
            "rust-src"
            "rust-analyzer"
          ])
          lldb
          cmake
          pkg-config
          openssl
          xorg.libxcb
          libxkbcommon
          vulkan-tools
          vulkan-headers
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          glslang
          udev
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    }
  );
}
