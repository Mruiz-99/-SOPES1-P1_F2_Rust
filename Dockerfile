# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.60.0

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

EXPOSE 2000
ENV DATABASE_URL mongodb://admindb:1234@35.209.237.73:27017
ENV DATABASE_NAME fase2
ENV USER_COLLECTION_NAME fase2
ENV SERVER_URL 0.0.0.0:2000
# Run the web service on container startup.
CMD ["cargo","run"]