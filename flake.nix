{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils, ... }:
	flake-utils.lib.eachDefaultSystem (system:
		let pkgs = import nixpkgs { inherit system; };
		in {
			packages.default = pkgs.rustPlatform.buildRustPackage {
				name = "lsfb2";
				version = "0.1.0";

				src = ./.;
				cargoLock.lockFile = ./Cargo.lock;

				buildPhase = "cargo build --release --features zip";
				installPhase = ''
					mkdir -p $out/bin
					cp target/release/lsfb2 $out/bin/
				'';
			};

			devShells.default = pkgs.mkShell {
				buildInputs = with pkgs; [
					rustc
					cargo

					rust-analyzer
				];

				shellHook = ''
					export HOME="$PWD/.nix-cache"
				'';
			};
		}
	);
}
