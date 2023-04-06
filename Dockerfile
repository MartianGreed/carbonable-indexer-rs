FROM --platform=linux/amd64 rust:1.68-slim-bullseye as builder

# Add unstable to packages list to install specific protobuf-compiler version
RUN echo "deb http://deb.debian.org/debian unstable main" >> /etc/apt/sources.list
RUN apt update && apt install pkg-config libssl-dev curl unzip protobuf-compiler=3.21.12-2 -y

WORKDIR /srv/www
COPY . .

RUN --mount=type=cache,target=/srv/www/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/carbonable-api ./carbonable-api

FROM --platform=linux/amd64 debian:bullseye-slim as production-runtime

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/; \
    echo "Installed base utils!"

WORKDIR /srv/www

COPY --from=builder /srv/www/carbonable-api ./carbonable-api

EXPOSE 8080

