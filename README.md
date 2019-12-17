# rust-musl-docker-example

Example project showing how to create a minimal Docker image for a Rust
application that requires SSL certificates.

The application is a webserver that proxies requests to httpbin.org over HTTPS.

The Dockerfile uses a multi-stage build, where the first stage compiles the app
using [`muslrust`](https://github.com/clux/muslrust), and the second stage
copies the build artifacts from the first stage over to a
[distroless base image](https://github.com/GoogleContainerTools/distroless/blob/master/base/README.md).

```sh
# Build and run
docker build -t rust-musl-docker-example .
docker run --init -d --name example -p8080:8080 --rm rust-musl-docker-example

# Test output
curl localhost:8080/anything

# Clean up
docker stop example
docker rmi rust-musl-docker-example
docker image prune --filter label=stage=intermediate
```

