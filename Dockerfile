FROM ekidd/rust-musl-builder:1.51.0 as build

WORKDIR /app

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

FROM scratch                                    

WORKDIR /app

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/tmp-wx-operator /app

CMD ["./tmp-wx-operator"]

