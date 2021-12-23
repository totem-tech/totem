# This is the build stage for Totem. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder
# Usage

# Totem Lego Test Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-lego-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-lego-node \
# --build-arg buildtype=build .

# Totem KAPEX Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-kapex-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-kapex-node \
# --build-arg buildtype=build .

# Totem WAPEX Westend Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-wapex-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=parachain-totem-wapex-node \
# --build-arg buildtype=build .

# Totem Pre-MainNet 

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=totem-mainnet-node \ 
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=totem-mainnet-node \
# --build-arg buildtype=build .

## select node type (see example above)
ARG chain

## Use check or build
ARG buildtype

ARG PROFILE=release
ARG CHAINPATH=p

WORKDIR /totem
COPY . /totem

RUN cargo "$buildtype" "--locked" "--$PROFILE" "-$CHAINPATH" "$chain"

# This is the 2nd stage: a very small image where we copy the Totem binary."
FROM docker.io/library/ubuntu:20.04

ARG chain

LABEL description="Multistage Docker image for Totem Live Accounting: a platform for web3" \
	totem.live.image.type="builder" \
	totem.live.image.authors="chris.dcosta@totemaccounting.com" \
	totem.live.image.vendor="Totem Accounting" \
	totem.live.image.description="Totem is a p2p accounting engine for the decentralised economy 🚀" \
	totem.live.image.source="https://gitlab.com/totem-tech/totem/totem_builder.Dockerfile" \
	totem.live.image.documentation="https://gitlab.com/totem-tech/totem"

COPY --from=builder /totem/target/release/"$chain" /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /totem totem && \
	mkdir -p /data /totem/.local/share/"$chain" && \
	chown -R totem:totem /data && \
	ln -s /totem/.local/share/"$chain" /data

# Sanity checks
RUN	ldd /usr/local/bin/"$chain" && \
	/usr/local/bin/"$chain" --version

# unclutter and minimize the attack surface
RUN	rm -rf /usr/bin /usr/sbin /usr/share/man

USER totem
# default substrate/parachain
EXPOSE 30333 9933 9944 9615

# polkadot internal to parachain
EXPOSE 40333 9934 9945 9616

VOLUME ["/data"]

CMD ["/usr/local/bin/$chain"]