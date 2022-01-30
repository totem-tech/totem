## select node type (see example above)
ARG chain

## Use check or build
ARG buildtype

# This is the build stage for Totem. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder
ARG chain
ARG buildtype
# Usage

# Totem Lego Test Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=lego-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=lego-node \
# --build-arg buildtype=build .

# Totem KAPEX Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=kapex-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=kapex-node \
# --build-arg buildtype=build .

# Totem WAPEX Westend Parachain

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=wapex-node \
# --build-arg buildtype=check .

# docker build \
# -f totem_builder.Dockerfile \
# --build-arg chain=wapex-node \
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

## constants
ARG PROFILE=release

WORKDIR /totem
COPY . /totem

# rust compiler command 
RUN cargo ${buildtype} --${PROFILE} -p ${chain}

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

# copy the binary from the builder image
#COPY --from=builder /totem/target/release/"${chain}" /usr/local/bin

# RUN useradd -m -u 1000 -U -s /bin/sh -d /totem totem && \
# 	mkdir -p /data /totem/.local/share && \
# 	chown -R totem:totem /data && \
# 	ln -s /data /totem/.local/share/totem
# RUN	chown -R totem:totem /data && \
# RUN	rm -rf /usr/bin /usr/sbin
# RUN	/usr/local/bin/"${chain}" --version


# create user, add shell as default, create home directory
RUN useradd -m -u 1000 -U -s /bin/sh -d /totem totem && \
# make a data directory for the runtime and the blockchain data
	mkdir -p {/totem/bin,/totem/.local/share/${chain}}

# copy the binary from the builder image into an appropriate runtime subdirectory
COPY --from=builder /totem/target/release/${chain} /totem/bin

# set permissions - so that the totem user owns everything (not root user)
RUN	chown -R totem:totem /totem

# create a symbolic link "/data" pointing to the standard blockchain data storage directory
RUN	ln -s /totem/.local/share/${chain} /data

# unclutter and minimize the attack surface
# delete all the bundled binaries on standard PATH locations
RUN rm -rf /bin /sbin /usr/bin /usr/sbin /usr/local/bin /usr/local/sbin

# Sanity checks
RUN	/totem/bin/${chain} --version

USER totem
# default substrate/parachain and polkadot internal to parachain
EXPOSE 30333 9933 9944 9615 40333 9934 9945 9616

# expose the directory for external persistant storage
VOLUME ["/data"]

# the executable
ENTRYPOINT ["/totem/bin"]