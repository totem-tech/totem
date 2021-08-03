# Note: We don't use Alpine and its packaged Rust/Cargo because they're too often out of date,
# preventing them from being used to build Substrate/Polkadot.

FROM phusion/baseimage:0.11 as builder
LABEL maintainer="chris.dcosta@totemaccounting.com"
LABEL description="This is the build stage for Totem Node Types. Here we create the binary."

ENV DEBIAN_FRONTEND=noninteractive

# Usage

# Totem Lego Parachain

# docker build \ 
# --build-arg chain=totem-lego-parachain-node \
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=totem-lego-parachain-node \
# --build-arg buildtype=build .

# Totem Mainnet 

# docker build \
# --build-arg chain=totem-mainnet-node \ 
# --build-arg buildtype=check .

# docker build \
# --build-arg chain=totem-mainnet-node \
# --build-arg buildtype=build .

ARG PROFILE=release
ARG CHAINPATH=p

## Use totem-lego-parachain-node or totem-mainnet-node
ARG chain

## Use check or build
ARG buildtype

WORKDIR /totem

COPY . /totem

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly-2021-03-01 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-01 && \
	rustup default nightly-2021-03-01 && \
    cargo "$buildtype" "--$PROFILE" "-$CHAINPATH" "$chain"

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="chris.dcosta@totemaccounting.com"
LABEL description="This is the 2nd stage: a very small image where we copy the Totem binary."
ARG chain

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	useradd -m -u 1000 -U -s /bin/sh -d /totem totemadmin && \
	mkdir -p /totem/.local/share/"$chain" && \
	chown -R totemadmin:totemadmin /totem/.local && \
	ln -s /totem/.local/share/"$chain" /data

COPY --from=builder /totem/target/release/"$chain" /usr/local/bin/

# checks
RUN ldd /usr/local/bin/"$chain" && \
	/usr/local/bin/"$chain" --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/bin /usr/sbin /usr/share/man

USER totemadmin
# default substrate
EXPOSE 30333 9933 9944 9615

# parachain
EXPOSE 30334 9934 9945 9616

VOLUME ["/data"]

CMD ["/usr/local/bin/$chain"]