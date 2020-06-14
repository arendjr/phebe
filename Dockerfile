FROM rust:1.44 as builder
WORKDIR /usr/src/phebe
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/phebe /opt/phebe/phebe
COPY --from=builder /usr/src/phebe/articles /opt/phebe/articles
CMD ["/opt/phebe/phebe"]
