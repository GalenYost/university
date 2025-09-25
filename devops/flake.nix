{
   inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
   };

   outputs = { self, nixpkgs }: let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
         inherit system;
         config.allowUnfree = true;
      };
   in {
      devShells.x86_64-linux.default = pkgs.mkShell {
         buildInputs = [
            pkgs.vagrant
            pkgs.ruby
            pkgs.ruby-lsp
         ];
         shellHook = ''
            export PS1='dev \W \$ '

            vagrant autocomplete install --bash
         '';
      };
   };
}
