# ====================
# Stage 1
# Build the API
# ====================
FROM rustlang/rust:nightly AS api_builder
WORKDIR /app
COPY . /app/
RUN cargo build --bin api --release

# ====================
# Stage Final
# Bundle API and Client into a single container
# ====================
FROM ubuntu:latest AS api_server
WORKDIR /app
COPY --from=api_builder /app/target/release/api .
# update for future dep install
RUN apt update
# Install libssl as dependency
RUN apt install libssl-dev -y
ENTRYPOINT ["./api"]
EXPOSE 8000/tcp