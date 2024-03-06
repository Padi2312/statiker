FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Use a distroless base image for the final image
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/statiker .

CMD ["/app/statiker -d /app/data"]
