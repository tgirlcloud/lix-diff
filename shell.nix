{
  mkShell,
  callPackage,
  rustPlatform,

  # extra tooling
  clippy,
  rustfmt,
  rust-analyzer,
}:
let
  defaultPackage = callPackage ./package.nix { };
in
mkShell {
  inputsFrom = [ defaultPackage ];

  env = {
    RUST_SRC_PATH = rustPlatform.rustLibSrc;
  };

  packages = [
    clippy
    rustfmt
    rust-analyzer
  ];
}
