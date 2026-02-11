FROM rust:alpine as builder

RUN apk add --no-cache musl-dev protobuf

WORKDIR /app

COPY . .

RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/kvwrap /kvwrap
CMD ["/kvwrap"]