{
   description = "sharp dev environment";

   inputs = {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
   };

   outputs = { self, nixpkgs }: let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
   in {
      devShells.x86_64-linux.default = pkgs.mkShell {
         buildInputs = [
            pkgs.csharp-ls
            pkgs.dotnet-sdk_9
            pkgs.dotnet-runtime_9
         ];
         shellHook = ''
            export DOTNET_ROOT=${pkgs.dotnet-sdk_9}

            export PS1='cs \W \$ '

            alias dotnet_run='dotnet run --property WarningLevel=0'
         '';
      };
   };
}
