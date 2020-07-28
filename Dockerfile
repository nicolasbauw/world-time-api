FROM ekidd/rust-musl-builder:nightly-2020-07-12 AS builder
RUN sudo apt update && sudo apt install -y upx
ENV ROCKET_ENV=production
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

# Size optimization
RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/world-time-api
RUN upx -9 /home/rust/src/target/x86_64-unknown-linux-musl/release/world-time-api

FROM alpine:latest
RUN apk update && apk add --no-cache tzdata
WORKDIR /usr/local/share/world-time-api
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/world-time-api \
    .
ENV ROCKET_ENV=production \
    ROCKET_LOG=off
RUN adduser -D -u 1001 runner; chown -R 1001 /usr/local/share/world-time-api
USER 1001
ENTRYPOINT ["./world-time-api"]