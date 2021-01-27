### builder ###
FROM rust:1.49 as builder
WORKDIR /usr/src

# setup the compiler toolchain (nightly + musl)
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup toolchain install nightly && \
    rustup target add x86_64-unknown-linux-musl --toolchain nightly

# download and build dependencies
RUN USER=root cargo new advent-of-code
WORKDIR /usr/src/advent-of-code
COPY Cargo.toml Cargo.lock rust-toolchain ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# build the executables
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

### executor ###
FROM scratch
WORKDIR /usr/src/advent-of-code
ENV PATH="/usr/src/advent-of-code:${PATH}"
COPY --from=builder /usr/local/cargo/bin/2020 /usr/local/cargo/bin/2019 ./
COPY data data

CMD ["2020"]
