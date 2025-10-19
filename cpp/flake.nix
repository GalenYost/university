{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.gnumake
          pkgs.ccls
          # pkgs.llvmPackages_latest.lldb
          # pkgs.llvmPackages_latest.libllvm
          # pkgs.llvmPackages_latest.libcxx
          # pkgs.llvmPackages_latest.clang
        ];
        shellHook = ''
          exec nu
        '';
      };
    };
}
