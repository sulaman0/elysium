# Node for Elysium
#
# Requires to run from repository root and to copy the binary in the build folder (part of the release workflow)

FROM debian:stable AS builder

# Branch or tag to build moonbeam from
ARG RUSTFLAGS=""
ENV RUSTFLAGS=$RUSTFLAGS
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /elysium

RUN echo "*** Installing Basic dependencies ***"
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
RUN apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler pkg-config

RUN set -e

COPY rust-toolchain.toml /elysium/rust-toolchain.toml

RUN echo "*** Installing Rust environment ***"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
#RUN rustup default stable
#RUN rustup install 1.82.0 && rustup default 1.82.0
# rustup version are pinned in the rust-toolchain file

# Print Rust and Cargo version
#RUN echo "==== RUST AND CARGO VERSION ====" && rustc --version && cargo --version
#RUN /root/.cargo/bin/rustc --version && /root/.cargo/bin/cargo --version

COPY . .

# Print target cpu
RUN rustc --print target-cpus

RUN echo "*** Building Elysium ***"
RUN cargo build --locked --release

FROM debian:stable-slim

LABEL description="Multistage Docker image for Elysium | The Green Blockchain for AI, Metaverse and web3 Game Projects" \
	io.parity.image.type="mainnet" \
	io.parity.image.authors="faraz.ahmad@vaivaltech.com" \
	io.parity.image.vendor="Elysium Foundation" \
	io.parity.image.description="Elysium | The Green Blockchain for AI, Metaverse and web3 Game Projects" \
	io.parity.image.source="https://github.com/elysium-foundation/elysium" \
	io.parity.image.documentation="https://docs.elysiumchain.tech/"

RUN useradd -m -u 1000 -U -s /bin/sh -d /elysium elysium && \
	mkdir -p /elysium/.local/share && \
	mkdir /data && \
	chown -R elysium:elysium /data && \
	ln -s /data /elysium/.local/share/elysium && \
	rm -rf /usr/sbin

USER elysium

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder --chown=elysium /elysium/target/release/elysium /elysium/elysium
COPY --from=builder --chown=elysium /elysium/elysiumSpecRaw.json /usr/local/bin/

RUN chmod uog+x /elysium/elysium
# 30333 for parachain p2p
# 30334 for relaychain p2p
# 9944 for Websocket & RPC call
# 9615 for Prometheus (metrics)
EXPOSE 30333 30334 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/elysium/elysium"]
