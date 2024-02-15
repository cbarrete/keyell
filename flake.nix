{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
   devShells.x86_64-linux.default = pkgs.mkShell rec {
     buildInputs = [
       pkgs.xorg.libX11
       pkgs.xorg.libXrandr
       pkgs.xorg.libXcursor
       pkgs.xorg.libXi
       pkgs.libxkbcommon
       pkgs.libGL
       pkgs.fontconfig
     ];

     LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
   };
  };
}
