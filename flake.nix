{
  description = "Frontend for scratch-test project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
        };
      in rec {
        apps.default = self.apps.${system}.example-server;
        apps.example-server = {
          program = "${(pkgs.writeShellScriptBin "scratch-test-web-example-server" ''
            ${pkgs.tree}/bin/tree ${packages.local-frontend}
            ${pkgs.caddy}/bin/caddy file-server --root ${packages.local-frontend}/dist --listen :8008
          '')}/bin/scratch-test-web-example-server";
          type = "app";
        };

        # version from Cargo.lock not in nixpkgs
        packages.wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
          src = pkgs.fetchCrate {
            pname = "wasm-bindgen-cli";
            version = "0.2.126";
            hash = "sha256-H6Is3fiZVxZCfOMWK5dWMSrtn50VGv0sfdnsT+cTtyk=";
          };

          cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
            inherit src;
            inherit (src) pname version;
            hash = "sha256-VucqkXbCi4qtQzY/HrXiDnbSURsagPsdNVMn1Tw3UiY=";
          };
        };

        packages.local-frontend = self.build.${system} {BACKEND_PREFIX = "http://localhost";};

        # BACKEND_PREFIX without trailing slash
        # BASE_URL with starting slash
        build = {
          BACKEND_PREFIX,
          BASE_URL ? "/",
        }:
          pkgs.rustPlatform.buildRustPackage {
            pname = "scratch-test-yew";
            version = "0.1.0";
            src = ./.;

            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
              dart-sass
              rust
              binaryen
              trunk
              # ensure Cargo.lock uses same version with `cargo update wasm-bindgen --precise ???`
              self.packages.${system}.wasm-bindgen-cli
            ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            inherit BACKEND_PREFIX;
            ROUTER_BASENAME = BASE_URL;

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "scratch-test-graph-analysis-0.1.0" = "sha256-eS5+D/1ysnf943W5rnoTG8KrVJtT9E6055q1S+xmZs0=";
                "scratch-test-interpreter-0.1.0" = "sha256-O3MS45yI0dxiAexsr1d7EVPfqHQH1LoHY5s+u6Zl/9M=";
                "scratch-test-model-0.1.0" = "sha256-IJr9UMl3kXnbKNw7FPT0k9kNxIlDwlbV2NgfQieJoOQ=";
                "scratch-test-value-0.1.0" = "sha256-s9CTHeoHne76kJbOpB7r8n4dDkWzHWiA/VUOq5zkDXE=";
                "scratch-test-report-0.1.0" = "sha256-gHPpjTKeMgp1+G6WxJhxAUkGIxaGi3qQaSmOwy0Ke2o=";
              };
            };

            buildPhase = ''
              trunk --skip-version-check build --offline --release --public-url ${BASE_URL}
            '';

            installPhase = ''
              mkdir -p $out
              mv dist $out/
            '';
          };
      }
    );
}
