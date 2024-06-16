{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    parts,
    nci,
    devshell,
    rust-overlay,
    nixpkgs,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [nci.flakeModule parts.flakeModules.easyOverlay devshell.flakeModule];
      perSystem = {
        config,
        pkgs,
        system,
        inputs',
        lib,
        self',
        ...
      }: let
        crateName = "learn-gl";
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs.${crateName};
        libPath = with pkgs;
          lib.makeLibraryPath
          [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            ecm
            glfw
          ];
      in rec {
        # use oxalica/rust-overlay
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
          config.allowUnfree = true;
        };

        # relPath is empty to denote current dir
        nci.projects.${crateName}.path = ./.;

        nci.crates.${crateName} = {
          # export crate (packages and devshell) in flake outputs
          export = true;

          # overrides
          drvConfig = {
            mkDerivation = {
              nativeBuildInputs = [pkgs.wayland-protocols pkgs.makeWrapper pkgs.libxkbcommon];
              buildInputs = [pkgs.pkg-config pkgs.openssl.dev pkgs.openssl pkgs.perl pkgs.SDL2];
              # postInstall = ''
              #   wrapProgram "$out/bin/learn-gl" --prefix LD_LIBRARY_PATH : "${libPath}"
              # '';
            };
          };

          # dependency overrides
          depsDrvConfig = {
            mkDerivation = {
              nativeBuildInputs = [pkgs.wayland-protocols pkgs.libxkbcommon pkgs.cmake pkgs.pkg-config pkgs.extra-cmake-modules];
              buildInputs = [pkgs.pkg-config pkgs.openssl.dev pkgs.openssl pkgs.perl pkgs.SDL2 pkgs.wayland];
            };
          };
          runtimeLibs = with pkgs; [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];
        };

        nci.toolchains = {
          build =
            pkgs.rust-bin.stable.latest.minimal;
        };

        devShells.default = crateOutputs.devShell.overrideAttrs (old:
          with pkgs; {
            packages =
              (old.packages or [])
              ++ [
                cmake
                (rust-bin.stable.latest.default.override {extensions = ["rust-src" "rust-analyzer"];})
              ];
          });

        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;

        # export overlay using easyOverlays
        overlayAttrs = {
          inherit (config.packages) learn-gl;
          /*
          inherit (inputs.rust-overlay.overlays) default;
          */
        };
        packages.learn-gl = crateOutputs.packages.release;
      };
      flake = {
        homeManagerModules = {
          learn-gl = import ./nix/hm-module.nix inputs.self;
          default = inputs.self.homeManagerModules.learn-gl;
        };
      };
    };
}
