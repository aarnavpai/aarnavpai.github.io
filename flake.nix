{
  description = "Basic devShell Flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { nixpkgs, ... }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = f: nixpkgs.lib.genAttrs systems (s: f s);
  in {
    devShells = forAllSystems (system: let 
      pkgs = import nixpkgs { inherit system; };
    in {
      default = pkgs.mkShell {
        name = "devShell";
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
          openssl
          trunk
        ];
      };
    });
  };
}
