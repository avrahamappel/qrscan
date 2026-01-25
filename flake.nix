{
  description =
    "qrscan - Scan a QR code in the terminal using the system camera or a given image";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "i686-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f:
        builtins.listToAttrs (map
          (name: {
            inherit name;
            value = f name;
          })
          systems);
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          version = builtins.concatStringsSep "-" [
            (builtins.substring 0 4 self.lastModifiedDate)
            (builtins.substring 4 2 self.lastModifiedDate)
            (builtins.substring 6 2 self.lastModifiedDate)
          ];
        in
        {
          qrscan = pkgs.rustPlatform.buildRustPackage {
            pname = "qrscan";
            inherit version;
            src = ./.;
            cargoLock = { lockFile = ./Cargo.lock; };
            nativeBuildInputs = [ pkgs.rustPlatform.bindgenHook ];
            doCheck = false; # No access to internet
          };
        });
      defaultPackage = forAllSystems (system: self.packages.${system}.qrscan);
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          inherit (pkgs) lib stdenv;
          devRequirements = with pkgs; [
            clippy
            cargo
            rustc
            rustfmt
            rust-analyzer
            openssl
            pkg-config
          ];
        in
        {
          default = pkgs.mkShell {
            RUST_BACKTRACE = 1;

            buildInputs = devRequirements;
            packages = devRequirements;
            nativeBuildInputs =
              [ pkgs.rustPlatform.bindgenHook ] ++
              lib.optional stdenv.isDarwin pkgs.zlib;
          };
        });
    };
}
