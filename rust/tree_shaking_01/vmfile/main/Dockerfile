ARG version=1.85-bookworm

FROM rust:$version

COPY ./dev_tracing.rs /repo
WORKDIR /repo
RUN printf '%s\n%s";' \
        'pub static BIG_TEXT: &str = "big text: 1234567890 * 10 * 10382' \
        "$(for i in `seq 1 10382`; do echo 1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890; done)" > ./src/common/big_text.rs && \
    du -h ./src/common/big_text.rs
RUN cargo build && cargo build --release && \
    mkdir -p /app/debug /app/release
RUN find_args="-mindepth 1 -maxdepth 1 -type f -perm -u+x" && \
    find ./target/debug $find_args -exec mv {} /app/debug \; && \
    find ./target/release $find_args -exec mv {} /app/release \; && \
    cargo clean
RUN printf "file: %s\n%s\n...\n---\n\nfile: %s\n%s\n---\n\n%s\n" \
        "/app/debug/code_3shaking_3print" \
        "`/app/debug/code_3shaking_3print | head -n 5`" \
        "/app/release/code_3shaking_3print" \
        "`/app/release/code_3shaking_3print`" \
        "du -ab /app/*" && \
        du -ab /app/*
CMD ["/app/release/code_3shaking_3print"]
