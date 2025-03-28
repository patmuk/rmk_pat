{
  description = "Rust Toolchain. Tested on Apple Silicon.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
        };
        overlays = [ ];
      };
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-Hn2uaQzRLidAWpfmRwSRdImifGUCAb9HeAqTYFXWeQk=";
      };
      # rmkit = pkgs.rustPlatform.buildRustPackage rec {
      #   pname = "cargo-rmkit";
      #   version = "0.0.11";

      #   buildInputs = with pkgs; [ openssl ];

      #   nativeBuildInputs = with pkgs; [ pkg-config ];

      #   src = pkgs.fetchFromGitHub {
      #     owner = "HaoboGu";
      #     repo = rmkit;
      #     rev = version;
      #     hash = "sha256-tHuT/dsiyliXdl34bFraYp3T3FUgxFnhEUQfc8O197I=";
      #   };

      #   cargoHash = "sha256-lUQwwGJLHLI9bfJiLUUE8j1svBAgbvr+8hKB/bRzwNA=";
      # };
    in
    {
      devShells.default = pkgs.mkShell
        {
          strictDeps = true;
          packages = with pkgs; [
            rustToolchain
            # rmkit
            # flip-link
          ];
          # shellHook = ''
          # '';
        };
    }
    );
}
