FROM rust:1-trixie AS build
WORKDIR /app
COPY ./ ./
RUN cargo build --release --locked

# Create a new stage for the final image
FROM debian:trixie-slim
WORKDIR /app
# Copy the binary from the build stage
COPY --from=build /app/target/release/spreet ./
ENTRYPOINT ["./spreet"]
