FROM rust:1.68 as builder
WORKDIR /usr/src/phebe
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/phebe /opt/phebe/phebe
WORKDIR /opt/phebe
CMD ["/opt/phebe/phebe"]

EXPOSE 3000
