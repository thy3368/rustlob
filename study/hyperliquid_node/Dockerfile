FROM ubuntu:24.04

ARG CHAIN=Mainnet
ARG HL_VERSION=
ARG HL_PUBLIC_KEY_URL=https://raw.githubusercontent.com/hyperliquid-dex/node/main/pub_key.asc
ENV DEBIAN_FRONTEND=noninteractive \
    CHAIN=${CHAIN} \
    HL_VERSION=${HL_VERSION} \
    HOME=/home/hyperliquid

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gnupg jq iproute2 iputils-ping net-tools procps tini \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --home-dir /home/hyperliquid --shell /bin/bash hyperliquid

WORKDIR /home/hyperliquid
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

RUN set -eux; \
    case "$CHAIN" in Mainnet|Testnet) ;; *) echo "invalid CHAIN=$CHAIN" >&2; exit 1 ;; esac; \
    base="https://binaries.hyperliquid.xyz/${CHAIN}"; \
    if [ -n "$HL_VERSION" ]; then base="$base/$HL_VERSION"; fi; \
    curl -fsSL "$base/hl-visor" -o /usr/local/bin/hl-visor; \
    curl -fsSL "$base/hl-visor.asc" -o /tmp/hl-visor.asc; \
    curl -fsSL "$HL_PUBLIC_KEY_URL" -o /tmp/pub_key.asc; \
    gpg --batch --import /tmp/pub_key.asc; \
    gpg --batch --verify /tmp/hl-visor.asc /usr/local/bin/hl-visor; \
    printf '{"chain":"%s"}\n' "$CHAIN" > /usr/local/bin/visor.json; \
    chmod 0755 /usr/local/bin/hl-visor /usr/local/bin/docker-entrypoint.sh; \
    rm -rf /root/.gnupg /tmp/hl-visor.asc /tmp/pub_key.asc; \
    mkdir -p /home/hyperliquid/hl/hyperliquid_data /home/hyperliquid/hl/data /home/hyperliquid/config; \
    chown -R hyperliquid:hyperliquid /home/hyperliquid; \
    chown hyperliquid:hyperliquid /usr/local/bin

USER hyperliquid
ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/docker-entrypoint.sh"]
