{
   description = "rust dev environment";

   inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
   };

   outputs = { self, nixpkgs }: let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
   in {
      devShells.x86_64-linux.default = pkgs.mkShell {
         buildInputs = [
            pkgs.cargo
            pkgs.rust-analyzer
         ];
         shellHook = ''
            export PS1='rs \W \$ '
            export PATH="$HOME/.cargo/bin:$PATH"
            export RUSTFLAGS="-A dead_code"
         '';
      };
   };
}
