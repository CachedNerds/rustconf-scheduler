FROM rustlang/rust:nightly

LABEL maintainer="Mackenzie Clark <mackenzie.a.z.c@gmail.com>"

# get the necessities
RUN apt-get update && apt-get install -y file git curl gcc make pkg-config libssl-dev cmake zlib1g-dev

RUN mkdir -p /rust

WORKDIR /rust

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-web

# Install base dependencies
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y -q --no-install-recommends \
        apt-transport-https \
        build-essential \
        ca-certificates \
        curl \
        git \
        libssl-dev \
        python \
        rsync \
        software-properties-common \
        devscripts \
        autoconf \
        ssl-cert \
    && apt-get clean

# update the repository sources list
# and install dependencies
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash -
RUN apt-get install -y nodejs

RUN npm i npm@latest -g

