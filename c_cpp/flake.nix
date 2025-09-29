{
   description = "gcc dev environment";

   inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
   };

   outputs = { self, nixpkgs }: let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
   in {
      devShells.x86_64-linux.default = pkgs.mkShell {
         buildInputs = [
            pkgs.libgcc
            pkgs.gdb
            pkgs.rocmPackages.clang
         ];
         shellHook = ''
            export PS1='gcc \W \$ '
            cpp_run() {
               local name="''${1:-main}"
               g++ "./$name.cpp" -o "./$name" && "./$name"
            }
         '';
      };
   };
}
