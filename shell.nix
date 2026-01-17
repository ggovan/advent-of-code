{
  unstable ? import <nixpkgs> {},
}:

with unstable;

mkShell {
  buildInputs = [
    awscli2
    hyperfine
    rustup
  ];
}
