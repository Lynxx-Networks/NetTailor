# Builder stage for compiling the Yew application
FROM rust:alpine3.19 as builder

# Install build dependencies
RUN apk update && apk upgrade && \
    apk add --no-cache musl-dev libffi-dev zlib-dev jpeg-dev

# Install wasm target and build tools
RUN rustup target add wasm32-unknown-unknown
    
RUN cargo install wasm-bindgen-cli

RUN apk update && apk upgrade

RUN echo "@testing http://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories

RUN apk add trunk@testing

# Add your application files to the builder stage
COPY . /app
WORKDIR /app/web

# Build the Yew application in release mode
RUN trunk build --release

# Final stage for setting up runtime environment
FROM alpine:3.19

# Metadata
LABEL maintainer="Collin Pendleton <collinp@collinpendleton.com>"

# Install runtime dependencies
RUN apk add --no-cache nginx openssh python3 openssl py3-pip bash mariadb-client postgresql-dev curl cronie openrc supervisor

# Setup Python environment
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Install Python packages
COPY --from=builder /app/requirements.txt /
RUN pip install --no-cache-dir -r /requirements.txt

# Copy wait-for-it script and give execute permission
COPY --from=builder /app/wait-for-it/wait-for-it.sh /wait-for-it.sh
RUN chmod +x /wait-for-it.sh

# Copy built files from the builder stage to the Nginx serving directory
COPY --from=builder /app/web/dist /var/www/html/

# Move to the root directory to execute the startup script
WORKDIR /

# Copy startup scripts
COPY startup/startup.sh /startup.sh
RUN chmod +x /startup.sh

# Copy nettailor runtime files
RUN mkdir -p /nettailor
RUN mkdir -p /var/log/supervisor/
COPY startup/ /nettailor/startup/
COPY clients/ /nettailor/clients/
COPY database_functions/ /nettailor/database_functions/
RUN chmod +x /nettailor/startup/startup.sh

ENV APP_ROOT /nettailor

# Configure Nginx
COPY startup/nginx.conf /etc/nginx/nginx.conf

# Start Nginx and keep it running
# CMD ["nginx", "-g", "daemon off;"]

ENTRYPOINT ["bash", "/startup.sh"]

