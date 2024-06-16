FROM docker.io/rust:alpine as builder
WORKDIR app

COPY . .
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static sqlite-dev sqlite-static
RUN cargo build --release --bin mollysocket


FROM alpine:3 as runtime
WORKDIR app

ENV MOLLY_HOST=127.0.0.1
ENV MOLLY_PORT=8020

RUN apk add --no-cache ca-certificates 

COPY --from=builder /app/target/release/mollysocket /usr/local/bin/
HEALTHCHECK --interval=1m --timeout=3s \
    CMD wget -q --tries=1 "http://$MOLLY_HOST:$MOLLY_PORT/" -O - | grep '"mollysocket":{"version":'
ENTRYPOINT ["/usr/local/bin/mollysocket"]
