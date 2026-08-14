FROM rust:alpine AS build

RUN apk add musl-dev
RUN mkdir -p /source

WORKDIR /source
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=build /source/target/x86_64-unknown-linux-musl/release/harmon-server /bin/harmon-server

CMD ["/bin/harmon-server", "run"]