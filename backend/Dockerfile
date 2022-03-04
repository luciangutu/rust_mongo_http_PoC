FROM rust:latest as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM rust:slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
EXPOSE 8080
CMD ["backend"]