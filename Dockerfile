FROM rust:alpine as builder

RUN apk add --no-cache git musl-dev protobuf

WORKDIR /app

COPY . .

RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/fjwrap /fjwrap
CMD ["/fjwrap"]