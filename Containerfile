FROM docker.io/rust:bookworm as builder
WORKDIR app
    
COPY . .
RUN cargo build --release --bin mollysocket


FROM docker.io/debian:bookworm as runtime
WORKDIR app

RUN apt update && \
    apt install -y libssl3 libsqlite3-0 ca-certificates


COPY --from=builder /app/target/release/mollysocket /usr/local/bin/
HEALTHCHECK --interval=5m --timeout=3s \
  CMD /usr/local/bin/mollysocket connection list
ENTRYPOINT ["/usr/local/bin/mollysocket"]
