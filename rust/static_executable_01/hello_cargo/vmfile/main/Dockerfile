
ARG version=1.80-bookworm

FROM rust:$version As builder

COPY . /repo
WORKDIR /repo
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
RUN pkgName=$(\
        cargo metadata --format-version=1 --no-deps | \
        grep -oE '"name":"[^"]*"' | \
        head -n 1 | sed -e 's/"name":"\(.*\)"/\1/' \
    ) && \
    mv "./target/release/$pkgName" /app
RUN cargo clean


FROM scratch
COPY --from=builder /app /app
CMD ["/app"]

