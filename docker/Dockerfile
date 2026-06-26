# Multi-stage build for the `obsidianlog` binary.
#
# Build context is the repository root:
#   docker build -f docker/Dockerfile -t obsidianlog .

FROM rust:1-slim AS build
WORKDIR /src
COPY . .
RUN cargo build --release --bin obsidianlog \
    && strip target/release/obsidianlog

FROM debian:stable-slim AS runtime
# Vector posts batches to the ingest endpoint on this port.
EXPOSE 7080
COPY --from=build /src/target/release/obsidianlog /usr/local/bin/obsidianlog
ENTRYPOINT ["obsidianlog"]
# Default to the ingest server; override the command for query/verify/init.
CMD ["ingest", "--bind", "0.0.0.0:7080"]
