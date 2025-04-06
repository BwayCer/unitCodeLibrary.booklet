ARG version=1.85-bookworm
ARG debianVersion=12


FROM rust:$version As builder
COPY ./link_lib.rs /repo
WORKDIR /repo
RUN cargo run --features bin-main --release --bin test_lib
RUN cargo build --lib --release && \
    cargo run --features headers --release \
        --bin generate_headers -- --lang c --file packages/my_app.c/src/LinkLib.h
RUN gcc packages/my_app.c/src/app_main.c -L ./target/release -l ffi_test -o myapp
    # -L: 指定庫文件所在的目錄.
    # -l: 指定庫文件的名稱. (會自動加上 `lib` 的前綴和 `.so` 擴展名)
RUN mkdir -p /app/lib && \
    mv ./target/release/libffi_test.so /app/lib/ && \
    mv ./myapp /app/ && \
    mv ./target/release/test_lib /app/


FROM debian:$debianVersion
COPY --from=builder /app /app
COPY --from=builder /app/test_lib /app.rs
ENV LD_LIBRARY_PATH="/app/lib;$LD_LIBRARY_PATH"
CMD ["/app/myapp"]
