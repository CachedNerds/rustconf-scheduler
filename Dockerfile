FROM rustlang/rust:nightly

LABEL maintainer="Mackenzie Clark <mackenzie.a.z.c@gmail.com>"

# get the necessities
RUN apt-get update && apt-get install -y file git curl gcc make pkg-config libssl-dev cmake zlib1g-dev

#ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz

RUN mkdir -p /rust

WORKDIR /rust

#RUN curl -f -L https://static.rust-lang.org/rustup.sh -O
#RUN sh ./rustup.sh --yes

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-web

#RUN curl -fsOSL $RUST_DOWNLOAD_URL \
#    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
#    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
#    && rm $RUST_ARCHIVE \
#    && ./install.sh
