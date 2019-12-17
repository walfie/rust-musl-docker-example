FROM clux/muslrust:1.39.0-stable as builder
LABEL stage=intermediate

COPY . /workspace
RUN set -x \
  && cd /workspace \
  && cargo build --release \
  && mv /workspace/target/*/release /out

FROM gcr.io/distroless/base
COPY --from=builder /out/rust-musl-docker-example /
USER nonroot

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/rust-musl-docker-example"]
EXPOSE 8080

