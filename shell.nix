with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustup

    pkgs.alsaLib
    pkgs.openssl
    pkgs.libudev
    pkgs.pkg-config
    systemd.dev

    # Example Build-time Additional Dependencies
    # pkg-config
    # systemd.dev
    # alsaLib
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    # openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
