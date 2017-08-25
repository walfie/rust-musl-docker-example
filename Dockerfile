FROM ekidd/rust-musl-builder as builder

USER rust
COPY . /home/rust/src/
RUN sudo apt-get update \
  && sudo apt-get install -y ca-certificates \
  && sudo update-ca-certificates \
  && cd /home/rust/src \
  && cargo build --release \
  && sudo mv \
    /home/rust/src/target/x86_64-unknown-linux-musl/release \
    /output

FROM scratch
EXPOSE 8080
COPY --from=builder /output/rust-musl-docker-example /
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENTRYPOINT ["/rust-musl-docker-example"]

