# build container
FROM rustlang/rust:nightly-slim as builder

# ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
# ENV CC_x86_64_unknown_linux_musl=clang
# ENV AR_x86_64_unknown_linux_musl=llvm-ar
# ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
# ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-x86_64 -L /usr/x86-64-linux-gnu"

# RUN rustup target add x86_64-unknown-linux-musl
# RUN apt update && apt install -y musl-tools musl-dev build-essential gcc-x86-64-linux-gnu clang llvm
# RUN update-ca-certificates

# RUN apt update && apt install -y librust-openssl-dev libssl-dev

# WORKDIR /app

# COPY . .

# RUN cargo build -p klever-sc-meta --target x86_64-unknown-linux-musl --release

RUN apt update && apt install -y --no-install-recommends clang llvm perl musl-tools
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

ENV CC_x86_64_unknown_linux_musl=clang
ENV AR_x86_64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV RUST_BACKTRACE=full

RUN \
  --mount=type=cache,target=/app/target,rw \
  --mount=type=cache,target=/usr/local/cargo/registry,rw \
  cd /app && \
  cargo build -p klever-sc-meta --target x86_64-unknown-linux-musl --release && \
  cp /app/target/x86_64-unknown-linux-musl/release/sc-meta /app/sc-meta


# target container
FROM alpine:3.17.3

WORKDIR /app

COPY --from=builder /app/sc-meta /app/sc-meta

# CMD ["./sc-meta", "--help"]
