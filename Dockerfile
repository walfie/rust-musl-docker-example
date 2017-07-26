FROM ekidd/rust-musl-builder as builder

USER rust
WORKDIR /home/rust/src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-musl-docker-example \
  /

ENTRYPOINT ["/rust-musl-docker-example"]

