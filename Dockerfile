FROM rust:1.44 as builder
WORKDIR /usr/src/phebe
COPY . .
RUN cargo install --locked --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/phebe /opt/phebe/phebe
COPY --from=builder /usr/src/phebe/articles /opt/phebe/articles
COPY --from=builder /usr/src/phebe/*.css /opt/phebe/
COPY --from=builder /usr/src/phebe/*.js /opt/phebe/
WORKDIR /opt/phebe
CMD ["/opt/phebe/phebe"]

EXPOSE 3000
