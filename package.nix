{ lib, rustPlatform }:
let
  toml = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "lix-diff";
  inherit (toml) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./src
      ]
    );
  };

  postInstall =
    # Create symlinks to multicall binary
    lib.concatMapStringsSep "\n" (bin: ''ln -sv "$out/bin/lemon-sorbet" "$out/bin/${bin}"'')
      # For each command in `lemon-sorbet`
      [
        "lix-diff"
      ];

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    inherit (toml) homepage description;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ isabelroses ];
    mainProgram = "lix-diff";
  };
}
