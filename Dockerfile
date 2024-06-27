FROM rust:1.78.0 as builder

WORKDIR /src

COPY . /src

RUN rustup target add $(rustup show active-toolchain | cut -d- -f2- | rev | cut -d- -f2- | rev)-musl

RUN cargo build --release --target=$(rustup show active-toolchain | cut -d- -f2- | rev | cut -d- -f2- | rev)-musl

WORKDIR /build

RUN cp /src/target/*-musl/release/semver ./

FROM scratch

LABEL org.opencontainers.image.source="https://github.com/nicholaschiasson/semver-extra"
LABEL org.opencontainers.image.description="A Rust implementation of the https://semver.org/ specification"
LABEL org.opencontainers.image.licenses="MIT"

COPY --from=builder --chmod=755 /build/semver /

ENTRYPOINT [ "/semver" ]
