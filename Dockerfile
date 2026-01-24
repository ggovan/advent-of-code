### builder ###
FROM rust:1.93 AS builder
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
COPY aoc-2021/Cargo.toml aoc-2021/Cargo.toml
COPY football-puzzle/Cargo.toml football-puzzle/Cargo.toml
COPY aoc-2025/Cargo.toml aoc-2025/Cargo.toml
RUN mkdir aoc-common/src && touch aoc-common/src/lib.rs && \
    mkdir aoc-2019/src && touch aoc-2019/src/lib.rs && \
    mkdir aoc-2020/src && touch aoc-2020/src/lib.rs && \
    mkdir aoc-2021/src && touch aoc-2021/src/lib.rs && \
    mkdir football-puzzle/src && touch football-puzzle/src/lib.rs && \
    mkdir aoc-2025/src && touch aoc-2025/src/lib.rs
RUN cargo install --target x86_64-unknown-linux-musl --path .

# remove the built version of the local crates, their cached versions make no sense
RUN rm /usr/src/advent-of-code/target/x86_64-unknown-linux-musl/release/deps/libaoc*

# copy the source
COPY aoc-common aoc-common/
COPY aoc-2019 aoc-2019/
COPY aoc-2020 aoc-2020/
COPY aoc-2021 aoc-2021/
COPY football-puzzle football-puzzle/
COPY aoc-2025 aoc-2025/
COPY src src/
# build the executables
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN strip /usr/local/cargo/bin/20*

### executor ###
FROM scratch
WORKDIR /usr/src/advent-of-code
ENV PATH="/usr/src/advent-of-code:${PATH}"
COPY --from=builder /usr/local/cargo/bin/20* ./
COPY data data

CMD ["2020"]
