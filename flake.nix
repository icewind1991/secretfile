{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        tools = with pkgs; [cargo rustc clippy bacon cargo-edit];
      in rec {
        devShell = pkgs.mkShell {
          nativeBuildInputs = tools;
        };
      }
    );
}
