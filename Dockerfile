FROM rust:latest AS build
WORKDIR /app
COPY ./ ./
RUN cargo build --release

# Create a new stage for the final image
FROM debian:latest
WORKDIR /app
# Copy the binary from the build stage
COPY --from=build /app/target/release/spreet ./
ENTRYPOINT ["./spreet"]