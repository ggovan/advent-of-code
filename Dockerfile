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
COPY aoc-common/Cargo.toml aoc-common/Cargo.toml
COPY aoc-2019/Cargo.toml aoc-2019/Cargo.toml
COPY aoc-2020/Cargo.toml aoc-2020/Cargo.toml
RUN mkdir aoc-common/src && touch aoc-common/src/lib.rs && \
    mkdir aoc-2019/src && touch aoc-2019/src/lib.rs && \
    mkdir aoc-2020/src && touch aoc-2020/src/lib.rs
RUN cargo install --target x86_64-unknown-linux-musl --path .

# remove the built version of the local crates, their cached versions make no sense
RUN rm /usr/src/advent-of-code/target/x86_64-unknown-linux-musl/release/deps/libaoc*

# build the executables
COPY aoc-common aoc-common/
COPY aoc-2020 aoc-2020/
COPY aoc-2019 aoc-2019/
COPY src src/
RUN cargo install --target x86_64-unknown-linux-musl --path .

### executor ###
FROM scratch
WORKDIR /usr/src/advent-of-code
ENV PATH="/usr/src/advent-of-code:${PATH}"
COPY --from=builder /usr/local/cargo/bin/2020 /usr/local/cargo/bin/2019 ./
COPY data data

CMD ["2020"]
