{
  pkgs,
  rustToolchain,
}:
pkgs.stdenv.mkDerivation rec {
  name = "axum-dev";

  # Compile time dependencies
  nativeBuildInputs = with pkgs; [
    # Rust
    rustToolchain
    rustPlatform.bindgenHook

    # Build
    pkg-config

    # SSL
    openssl
    openssl.dev

    # Linker
    wild

    # Bindgen
    clang
    llvmPackages.libclang.lib
  ];

  # Rust variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  # Bindgen variables
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
    nativeBuildInputs
    ++ [
      pkgs.llvmPackages.llvm
    ]
  );
}
