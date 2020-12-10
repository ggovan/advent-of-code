{ 
  unstable ? import <nixpkgs> {},
}:

with unstable;

mkShell {
  buildInputs = [
    hyperfine
    rustup
  ];
}
