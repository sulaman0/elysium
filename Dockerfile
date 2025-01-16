# This is the build stage for elysium. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production AS builder
WORKDIR /elysium
COPY . /elysium

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the elysium binary."
# FROM docker.io/library/ubuntu:latest
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Elysium | The Green Blockchain for AI, Metaverse and web3 Game Projects" \
	io.parity.image.type="mainnet" \
	io.parity.image.authors="faraz.ahmad@vaivaltech.com" \
	io.parity.image.vendor="BloxBytes" \
	io.parity.image.description="Elysium | The Green Blockchain for AI, Metaverse and web3 Game Projects" \
	io.parity.image.source="https://hub.docker.com/repository/docker/intellicoworks" \
	io.parity.image.documentation="https://docs.elysiumchain.tech/"

COPY --from=builder /elysium/target/release/elysium /usr/local/bin

COPY elysiumSpecRaw.json /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /elysium elysium && \
	mkdir -p /data /elysium/.local/share && \
	chown -R elysium:elysium /data && \
	ln -s /data /elysium/.local/share/elysium && \
    # unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin

USER elysium

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/elysium"]