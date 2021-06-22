with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    # Example Build-time Additional Dependencies
    # pkg-config
    # systemd.dev
    # alsaLib
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    # openssl
    
    rustc
    cargo
    alsaLib
    openssl
    libudev
    pkg-config
    systemd.dev
    lldb
    gcc
    glibc
    clang
    pkgconfig
    capnproto
    gnumake
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
