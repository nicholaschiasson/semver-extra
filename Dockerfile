FROM rust:1.78.0 as builder

WORKDIR /src

COPY . /src

RUN rustup target add $(rustup show active-toolchain | cut -d- -f2- | rev | cut -d- -f2- | rev)-musl

RUN cargo build --release --target=$(rustup show active-toolchain | cut -d- -f2- | rev | cut -d- -f2- | rev)-musl

WORKDIR /build

RUN cp /src/target/*-musl/release/semver ./

FROM scratch

COPY --from=builder --chmod=755 /build/semver /

ENTRYPOINT [ "/semver" ]
