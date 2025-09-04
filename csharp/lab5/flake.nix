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
            pkgs.dotnet-sdk_9
            pkgs.dotnet-runtime_9
         ];
         shellHook = ''
            if [ ! -x ./.tools/csharp-ls ]; then
               mkdir -p ./.tools
               dotnet tool install --tool-path ./.tools csharp-ls
            fi

            export DOTNET_ROOT=${pkgs.dotnet-sdk_9}
            export PATH=$DOTNET_ROOT/bin:$PATH
            export PATH=$PWD/.tools:$PATH
            export PS1='cs-dev \W\$ '

            alias dotnet_run='dotnet run --property WarningLevel=0'
         '';
      };
   };
}
