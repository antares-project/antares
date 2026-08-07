FROM rust:alpine AS build

RUN apk add musl-dev
RUN mkdir -p /source

WORKDIR /source
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=build /source/target/x86_64-unknown-linux-musl/release/antares-server /bin/antares-server

CMD ["/bin/antares-server"]