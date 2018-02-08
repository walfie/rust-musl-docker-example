# rust-musl-docker-example

Example project showing how to create a minimal Docker image for a Rust
application that requires SSL certificates.

The application is a webserver that proxies requests to httpbin.org over HTTPS.

The Dockerfile uses a multi-stage build, where the first stage compiles the app
using [`muslrust`](https://github.com/clux/muslrust), and the second stage
copies the build artifacts (and SSL certificates) from the first stage over to
a `FROM scratch` base image.

```sh
# Build and run
docker build -t rust-musl-docker-example .
docker run --init -d --name example -p8080:8080 --rm rust-musl-docker-example

# Test output
curl localhost:8080/anything

# Clean up
docker stop example
docker rmi rust-musl-docker-example
```

