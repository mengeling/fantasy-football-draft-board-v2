
{ pkgs }: {
  deps = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.google-chrome
    pkgs.chromium
  ];
}
