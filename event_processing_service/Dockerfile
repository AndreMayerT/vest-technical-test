FROM rust:1.70.0 as builder

WORKDIR /usr/src/event_processing_service
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/event_processing_service /usr/local/bin/event_processing_service
CMD ["event_processing_service"]
