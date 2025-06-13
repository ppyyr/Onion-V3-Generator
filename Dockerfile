# Multi-stage Dockerfile for cross-platform build (arm64 -> amd64)
# Build stage
FROM python:3.11-slim AS builder

# Install build dependencies for cryptography and other native extensions
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    libc6-dev \
    libffi-dev \
    libssl-dev \
    cargo \
    rustc \
    pkg-config \
    && pip install --upgrade pip \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy requirements first for better caching
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir --user -r requirements.txt

# Production stage
FROM python:3.11-slim AS production

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libffi8 \
    libssl3 \
    && groupadd -g 1000 onion \
    && useradd -d /home/onion -s /bin/bash -u 1000 -g onion onion \
    && mkdir -p /home/onion \
    && chown onion:onion /home/onion \
    && rm -rf /var/lib/apt/lists/*

# Copy installed packages from builder stage
COPY --from=builder /root/.local /home/onion/.local

# Set working directory
WORKDIR /app

# Copy application code
COPY --chown=onion:onion onion_generator/ ./onion_generator/

# Switch to non-root user
USER onion

# Make sure scripts in .local are usable
ENV PATH=/home/onion/.local/bin:$PATH

# Set Python path
ENV PYTHONPATH=/app

# Expose any ports if needed (none for this CLI app)
# EXPOSE 8080

# Health check (optional)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import onion_generator" || exit 1

# Set environment variables for better Docker experience
ENV PYTHONUNBUFFERED=1
ENV PYTHONIOENCODING=utf-8

# Default command
ENTRYPOINT ["python", "-m", "onion_generator"]

# Default arguments (can be overridden)
CMD ["--help"]
