FROM rust:1.70 as builder

WORKDIR /usr/src/todo-api
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/todo-api/target/release/todo-api /usr/local/bin/todo-api

# Copy the .env file
COPY .env .env

# Expose the API port
EXPOSE 8080

CMD ["todo-api"]