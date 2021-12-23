# This is the build stage for Totem. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder
# Usage

# Totem Lego Test Parachain

# docker build \ 
# --build-arg chain=parachain-totem-lego-node \
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=parachain-totem-lego-node \
# --build-arg buildtype=build .

# Totem KAPEX Parachain

# docker build \ 
# --build-arg chain=parachain-totem-kapex-node \
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=parachain-totem-kapex-node \
# --build-arg buildtype=build .

# Totem WAPEX Westend Parachain

# docker build \ 
# --build-arg chain=parachain-totem-wapex-node \
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=parachain-totem-wapex-node \
# --build-arg buildtype=build .

# Totem Pre-MainNet 

# docker build \
# --build-arg chain=totem-mainnet-node \ 
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=totem-mainnet-node \
# --build-arg buildtype=build .

ARG PROFILE=release
ARG CHAINPATH=p

## select node type (see example above)
ARG chain

## Use check or build
ARG buildtype

WORKDIR /totem
COPY . /totem

RUN cargo "$buildtype" "--locked" "--$PROFILE" "-$CHAINPATH" "$chain"
# RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Totem binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Multistage Docker image for Totem Live Accounting: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chris.dcosta@totemaccounting.com" \
	io.parity.image.vendor="Totem Accounting" \
	io.parity.image.description="Totem is a p2p accounting engine for the decentralised economy 🚀" \
	io.parity.image.source="https://gitlab.com/totem-tech/totem/totem_builder.Dockerfile" \
	io.parity.image.documentation="https://gitlab.com/totem-tech/totem"

COPY --from=builder /totem/target/release/"$chain" /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /totem totem && \
	mkdir -p /data /totem/.local/share/"$chain" && \
	chown -R totem:totem /data && \
	ln -s /data /totem/.local/share/"$chain" && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# Sanity checks
	ldd /usr/local/bin/"$chain" && \
	/usr/local/bin/"$chain" --version

USER totem
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]