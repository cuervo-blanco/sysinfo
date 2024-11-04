FROM rust:1.78-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM python:3.12.3-slim

RUN apt-get update && \
    apt-get install -y libgl1-mesa-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt .
RUN python3 -m venv /app/venv && \
    /app/venv/bin/pip install --upgrade pip && \
    /app/venv/bin/pip install -r requirements.txt

COPY --from=builder /app/target/release/sysinfo /app/sysinfo

ENV PATH="/app/venv/bin:/app:$PATH"

ENTRYPOINT ["/app/sysinfo"]

