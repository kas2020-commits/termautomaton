{rustPlatform, ... }:
rustPlatform.buildRustPackage {
  pname = "termataumaton";
  version = "0.1.0";
  src = ./.;
  cargoSha256 = "sha256-Q1TkDkuWfRO6Jj0HawagQL9VL0pA+eDZ9knKuT2exM0=";
}
