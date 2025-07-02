FROM rust:latest AS build
WORKDIR /usr/src/app
COPY ./ ./
RUN cargo build --release

# Create a new stage for the final image
FROM debian:latest
WORKDIR /usr/src/app
# Copy the binary from the build stage
COPY --from=build /usr/src/app/target/release/spreet ./
CMD ["./spreet"]