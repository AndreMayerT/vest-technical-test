FROM rust:1.70.0 as builder

WORKDIR /usr/src/api_service
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/api_service /usr/local/bin/api_service
CMD ["api_service"]