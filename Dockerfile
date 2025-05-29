# Build stage
FROM rust:1.87-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libudev-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user and add to plugdev group
RUN groupadd -r plugdev || true && \
    useradd -r -s /bin/false -G plugdev appuser

# Copy the binary from builder stage
COPY --from=builder /app/target/release/AirCtrlMqtt /usr/local/bin/airctrl-mqtt

# Change to non-root user
USER appuser

# Set environment variables with defaults
ENV MQTT_HOST=localhost
ENV MQTT_PORT=1883
ENV MQTT_CLIENT_ID=airctrl_client
ENV MQTT_TOPIC=airctrl/sensors

ENTRYPOINT ["airctrl-mqtt"]