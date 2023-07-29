FROM docker.io/rust:buster as builder
WORKDIR app
    
COPY . .
RUN cargo build --release --bin mollysocket


FROM docker.io/debian:buster as runtime
WORKDIR app

RUN apt update && \
    apt install -y libssl1.1 libsqlite3-0


COPY --from=builder /app/target/release/mollysocket /usr/local/bin/
HEALTHCHECK --interval=5m --timeout=3s \
  CMD /usr/local/bin/mollysocket connection list
ENTRYPOINT ["/usr/local/bin/mollysocket"]
