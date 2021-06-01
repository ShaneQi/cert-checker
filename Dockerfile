# Build Stage
FROM rust:1.52.1 AS builder
WORKDIR /src/
COPY . /src
RUN cargo build --release

# Bundle Stage
FROM nginx:1.21.0
COPY --from=builder /src/target/release/cert-checker .
USER 1000
CMD ["/bin/bash"]