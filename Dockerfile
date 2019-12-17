FROM clux/muslrust:1.39.0-stable as builder
LABEL stage=intermediate

COPY . /workspace
RUN set -x \
  && apt-get update \
  && apt-get install -y ca-certificates \
  && update-ca-certificates \
  && cd /workspace \
  && cargo build --release \
  && mv /workspace/target/*/release /out

FROM scratch
COPY --from=builder /out/rust-musl-docker-example /
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/rust-musl-docker-example"]
EXPOSE 8080

